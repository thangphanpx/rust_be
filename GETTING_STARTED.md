# Hướng dẫn chạy dự án Rust Backend

## Yêu cầu hệ thống

- Rust 1.75 trở lên
- PostgreSQL 12 trở lên
- Docker (tùy chọn)

## Cách 1: Chạy với Docker (Khuyến nghị)

### Bước 1: Khởi động database và ứng dụng
```bash
docker-compose up -d
```

### Bước 2: Kiểm tra logs
```bash
docker-compose logs -f app
```

### Bước 3: Truy cập ứng dụng
- API: http://localhost:3000/api/v1/health
- Swagger UI: http://localhost:3000/swagger-ui/

## Cách 2: Chạy local

### Bước 1: Cài đặt PostgreSQL
Tạo database:
```sql
CREATE DATABASE rust_be_db;
```

### Bước 2: Cấu hình môi trường
```bash
cp .env.example .env
```

Chỉnh sửa file `.env` với thông tin database của bạn:
```env
DATABASE_URL=postgresql://username:password@localhost:5432/rust_be_db
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=your-secret-key-here
RUST_LOG=debug
```

### Bước 3: Chạy migration (nếu có)
```bash
# Nếu có SeaORM CLI
sea-orm-cli migrate up

# Hoặc chạy SQL script trực tiếp
psql -d rust_be_db -f init.sql
```

### Bước 4: Build và chạy ứng dụng
```bash
cargo build --release
cargo run
```

## Kiểm tra API

### Health Check
```bash
curl http://localhost:3000/api/v1/health
```

### Tạo user mới
```bash
curl -X POST http://localhost:3000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "password123",
    "full_name": "Test User"
  }'
```

### Lấy danh sách users
```bash
curl http://localhost:3000/api/v1/users
```

### Tạo post mới
```bash
curl -X POST http://localhost:3000/api/v1/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Hello World",
    "content": "This is my first post!",
    "is_published": true
  }'
```

### Lấy danh sách posts
```bash
curl http://localhost:3000/api/v1/posts
```

## Truy cập Swagger UI

Mở trình duyệt và truy cập: http://localhost:3000/swagger-ui/

Tại đây bạn có thể:
- Xem tất cả API endpoints
- Test API trực tiếp từ giao diện web
- Xem schema của các model

## Troubleshooting

### Database connection error
- Kiểm tra PostgreSQL đang chạy
- Kiểm tra thông tin kết nối trong file `.env`
- Đảm bảo database đã được tạo

### Port đã được sử dụng
- Thay đổi `SERVER_PORT` trong file `.env`
- Hoặc dừng process đang sử dụng port 3000

### Docker issues
```bash
# Xóa containers và rebuild
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

## Phát triển

### Thêm API endpoint mới
1. Tạo model trong `src/models/`
2. Tạo handler trong `src/handlers/`
3. Thêm route trong `src/main.rs`
4. Cập nhật OpenAPI documentation

### Database migration
1. Tạo migration file trong `migration/src/`
2. Cập nhật `migration/src/lib.rs`
3. Chạy migration

### Testing
```bash
cargo test
```