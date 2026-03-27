#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// Cấu trúc dữ liệu của một Lời chúc (Memory)
#[contracttype]
#[derive(Clone)]
pub struct Memory {
    pub sender: Address, // Địa chỉ ví của người viết lưu bút
    pub message: String, // Nội dung lời chúc
}

// Định nghĩa khóa lưu trữ trên blockchain
#[contracttype]
pub enum DataKey {
    // Lưu trữ danh sách lời chúc theo địa chỉ ví của người nhận
    Yearbook(Address), 
}

#[contract]
pub struct YearbookContract;

#[contractimpl]
impl YearbookContract {
    
    /// Hàm để gửi một lời chúc mới
    /// `sender`: Người gửi lời chúc
    /// `receiver`: Người nhận lời chúc (chủ nhân cuốn lưu bút)
    /// `message`: Nội dung
    pub fn write_memory(env: Env, sender: Address, receiver: Address, message: String) {
        // 1. Yêu cầu người gửi phải ký xác nhận giao dịch (bảo mật, chống mạo danh)
        sender.require_auth();

        // 2. Lấy danh sách lời chúc hiện tại của người nhận từ bộ nhớ Storage
        // Nếu chưa có ai gửi lời chúc nào, tạo một mảng (Vec) rỗng
        let mut memories: Vec<Memory> = env
            .storage()
            .persistent()
            .get(&DataKey::Yearbook(receiver.clone()))
            .unwrap_or(Vec::new(&env));

        // 3. Thêm lời chúc mới vào danh sách
        memories.push_back(Memory { sender, message });

        // 4. Lưu danh sách đã cập nhật trở lại blockchain
        env.storage()
            .persistent()
            .set(&DataKey::Yearbook(receiver), &memories);
    }

    /// Hàm để xem tất cả lời chúc của một người
    pub fn get_memories(env: Env, receiver: Address) -> Vec<Memory> {
        // Chỉ cần lấy dữ liệu từ Storage ra và trả về
        env.storage()
            .persistent()
            .get(&DataKey::Yearbook(receiver))
            .unwrap_or(Vec::new(&env))
    }
}
