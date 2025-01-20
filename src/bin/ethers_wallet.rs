use ethers::{
    signers::{LocalWallet, Signer, Wallet},
    utils::keccak256,
};
use std::{fs, path::PathBuf};

fn main() {}

async fn get_wallet(address: &str, password: &str) -> LibResult<LocalWallet> {
    let keystore_path = PathBuf::from(consts::KEYSTORE_DIR.as_str());
    // 处理地址格式
    let clean_address = address.trim_start_matches("0x").to_lowercase();
    // 读取目录下的所有文件
    let entries = fs::read_dir(&keystore_path)
        .map_err(|e| LibError::ParamError(format!("Failed to read keystore directory: {}", e)))?;
    // 查找匹配地址的 keystore 文件
    let keystore_file = entries
        .filter_map(Result::ok)
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .ends_with(&clean_address)
        })
        .ok_or_else(|| LibError::ParamError("Keystore file not found".to_string()))?;
    // 使用 ethers 内置的 Wallet::decrypt_keystore
    let wallet = Wallet::decrypt_keystore(keystore_file.path(), password)
        .map_err(|e| LibError::ParamError(format!("Failed to decrypt keystore: {}", e)))?;

    Ok(wallet)
}

pub async fn launch_token(user_address: String) -> LibResult<String> {
    // 生成签名消息
    let message = ethers::abi::encode(&[
        ethers::abi::Token::Address(user_address.parse().unwrap()),
        ethers::abi::Token::Uint(1.into()),
        ethers::abi::Token::Uint(1.into()),
    ]);

    let message_hash = keccak256(message);

    // 直接使用私钥创建钱包
    let wallet = get_wallet(&"111", "123").await?;

    let signature = wallet.sign_message(&message_hash).await?;

    Ok(signature)
}
