# Rust Backend API

Một REST API được xây dựng với Rust sử dụng Tokio, Axum, SeaORM, Serde và utoipa.

## 🚀 Tính năng

- **Tokio** - Runtime bất đồng bộ cho Rust
- **Axum** - Web framework hiện đại và ergonomic
- **SeaORM** - Async & dynamic ORM for Rust
- **Serde** - Serialization framework cho Rust
- **utoipa** - Auto-generate OpenAPI documentation
- **PostgreSQL** - Database chính
- **Docker** - Containerization
- **Swagger UI** - Interactive API documentation

## 📁 Cấu trúc dự án

```
rust_be/
├── src/
│   ├── config/          # Cấu hình ứng dụng
│   ├── database/        # Kết nối và setup database
│   ├── entities/        # SeaORM entities
│   ├── handlers/        # HTTP request handlers
│   ├── models/          # Request/Response models
│   └── main.rs          # Entry point
├── migration/           # Database migrations
├── Cargo.toml          # Dependencies
├── Dockerfile          # Docker image
├── docker-compose.yml  # Multi-container setup
└── README.md
```

## 🛠️ Cài đặt

### Yêu cầu

- Rust 1.75+
- PostgreSQL 12+
- Docker (optional)

### Chạy local

1. Clone repository:
```bash
git clone <repository-url>
cd rust_be
```

2. Cài đặt dependencies:
```bash
cargo build
```

3. Setup database:
```bash
# Tạo database PostgreSQL
createdb rust_be_db

# Copy environment file
cp .env.example .env

# Chỉnh sửa .env với thông tin database của bạn
```

4. Chạy ứng dụng:
```bash
cargo run
```

### Chạy với Docker

1. Chạy toàn bộ stack:
```bash
docker-compose up -d
```

2. Xem logs:
```bash
docker-compose logs -f app
```

## 📚 API Documentation

Sau khi chạy ứng dụng, bạn có thể truy cập:

- **Swagger UI**: http://localhost:3000/swagger-ui/
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

## 🎯 API Endpoints

### Health Check
- `GET /api/v1/health` - Health check

### Users
- `GET /api/v1/users` - Lấy danh sách users (có phân trang)
- `POST /api/v1/users` - Tạo user mới
- `GET /api/v1/users/{id}` - Lấy user theo ID
- `PUT /api/v1/users/{id}` - Cập nhật user
- `DELETE /api/v1/users/{id}` - Xóa user

### Posts
- `GET /api/v1/posts` - Lấy danh sách posts (có phân trang)
- `POST /api/v1/posts` - Tạo post mới
- `GET /api/v1/posts/{id}` - Lấy post theo ID
- `PUT /api/v1/posts/{id}` - Cập nhật post
- `DELETE /api/v1/posts/{id}` - Xóa post

## 🔧 Configuration

Ứng dụng sử dụng các biến môi trường:

```env
DATABASE_URL=postgresql://user:password@localhost:5432/database
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=your-secret-key
RUST_LOG=debug
```

## 🧪 Testing

Chạy tests:
```bash
cargo test
```

## 📝 Development

### Thêm migration mới

1. Tạo migration:
```bash
sea-orm-cli migrate generate <migration_name>
```

2. Chạy migration:
```bash
sea-orm-cli migrate up
```

### Generate entities từ database

```bash
sea-orm-cli generate entity -o src/entities
```

## 🐳 Docker Commands

```bash
# Build image
docker build -t rust_be .

# Run container
docker run -p 3000:3000 rust_be

# Run with docker-compose
docker-compose up -d

# Stop containers
docker-compose down

# View logs
docker-compose logs -f
```

## 📄 License

MIT License - xem file [LICENSE](LICENSE) để biết thêm chi tiết.

## 🤝 Contributing

1. Fork repository
2. Tạo feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Tạo Pull Request

## 📞 Support

Nếu bạn có câu hỏi hoặc cần hỗ trợ, vui lòng tạo issue trong repository này.