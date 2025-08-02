# QR Code Generator API

A fast, lightweight, and production-ready QR code generation API built with Rust and Axum. Generate QR codes from URLs or text with support for both JSON responses and direct PNG downloads.

![QR Code API](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)
![Version](https://img.shields.io/badge/Version-2.0.0-blue.svg)

### Sample QR Code
![Sample QR Code](assets/sample_qr.png)
*This QR code links to the GitHub repository*

## 🚀 Features

### Core Features
- **Fast QR Generation**: Built with Rust for optimal performance
- **Multiple Output Formats**: PNG, SVG, and JPEG support
- **CORS Enabled**: Ready for web applications
- **Production Ready**: Includes logging, error handling, and validation
- **Docker Support**: Easy deployment with Docker
- **Configurable**: Environment-based configuration

### ✨ New in v2.0.0
- **🎨 Customizable QR Codes**: Control size, colors, and error correction
- **📏 Multiple Sizes**: Small (150px), Medium (300px), Large (600px), or Custom
- **🔒 Error Correction Levels**: L (~7%), M (~15%), Q (~25%), H (~30%)
- **🎯 Color Customization**: Set custom foreground/background colors
- **✅ Enhanced Validation**: URL format checking and security validation
- **🔄 Backward Compatible**: V1 endpoints remain fully functional
- **📊 Multiple API Styles**: POST with JSON or GET with query parameters

> 📋 **Roadmap**: See [ROADMAP.md](ROADMAP.md) for upcoming features and development plans

## 📋 API Endpoints

### V1 Endpoints (Legacy)

#### Health Check
```http
GET /
```
Returns API status and available endpoints.

#### Generate QR Code (JSON)
```http
GET /generate?url=<your-url>
```
Returns QR code as base64-encoded PNG in JSON format.

**Response:**
```json
{
  "qr_code": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQgAAAEICAAAAACGnTUjAAAFtUlEQVR4Ae3AA6AkWZbG8f937o3IzKdyS2Oubdu2bdu2bdu2bWmMnpZKr54yMyLu+Xa3anqmhztr1a+aqwAqVwFQuQqAylUAVK4CoHIVAJWrAKhcBUDlKgAqVwFQuQqAylUAVK4CoHIVAJWrAKhcBUDlKgAqVwFQuQqAylUAVK4CoHIVAJWrAKhcBUDlKgAqVwFQuQqAylUAVK4CoHIVAJWrAKhcBUDlXyD+bcwV4gpzhbjCXCG...",
  "format": "png"
}
```

#### Generate QR Code (PNG Download)
```http
GET /image?url=<your-url>
```
Returns downloadable PNG image directly.

### V2 Endpoints (Enhanced) 🆕

#### Generate Customized QR Code (JSON)
```http
POST /v2/generate
Content-Type: application/json

{
  "url": "https://example.com",
  "customization": {
    "size": "medium",              // "small", "medium", "large", or number
    "error_correction": "M",       // "L", "M", "Q", "H"
    "colors": {
      "foreground": "#000000",     // Hex color for QR code
      "background": "#FFFFFF"      // Hex color for background
    },
    "border_width": 4,             // Border size in pixels
    "format": "png"                // "png", "svg", "jpeg"
  }
}
```

**Response:**
```json
{
  "qr_code": "data:image/png;base64,iVBORw0KGgo...",
  "format": "png",
  "size": "medium (300px)",
  "error_correction": "M",
  "colors": {
    "foreground": "#000000",
    "background": "#FFFFFF"
  },
  "border_width": 4
}
```

#### Generate Customized QR Code (Query Parameters)
```http
GET /v2/generate?url=<url>&size=<size>&format=<format>&error_correction=<level>&foreground_color=<hex>&background_color=<hex>&border_width=<number>
```

#### Generate Customized QR Code (Image Download)
```http
POST /v2/image
Content-Type: application/json

{
  "url": "https://example.com",
  "customization": { ... }
}
```
Returns downloadable image file in specified format.

## 🛠️ Installation & Usage

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Local Development

1. **Clone the repository**
```bash
git clone https://github.com/Tolstoyj/QRCodeGeneratorApi.git
cd QRCodeGeneratorApi
```

2. **Install dependencies**
```bash
cargo build
```

3. **Run the server**
```bash
cargo run
```

The API will be available at `http://localhost:3000`

### Environment Variables

Create a `.env` file (optional):
```env
HOST=0.0.0.0
PORT=3000
LOG_LEVEL=info
MAX_URL_LENGTH=2048
```

### Railway Deployment (Recommended)

1. **Install Railway CLI**
```bash
npm install -g @railway/cli
```

2. **Login to Railway**
```bash
railway login
```

3. **Deploy to Railway**
```bash
railway init
railway up
```

4. **Set Environment Variables** (Optional)
```bash
railway variables set HOST=0.0.0.0
railway variables set PORT=3000
railway variables set LOG_LEVEL=info
railway variables set MAX_URL_LENGTH=2048
```

### Docker Deployment

1. **Build the image**
```bash
docker build -t qr-code-api .
```

2. **Run the container**
```bash
docker run -p 3000:3000 qr-code-api
```

## 📖 Examples

### Using cURL

#### V1 API (Legacy)
```bash
# Generate QR code as JSON
curl "http://localhost:3000/generate?url=https://github.com"

# Download QR code as PNG
curl "http://localhost:3000/image?url=https://github.com" -o qr_code.png
```

#### V2 API (Enhanced)
```bash
# Generate customized QR code (POST)
curl -X POST "http://localhost:3000/v2/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com",
    "customization": {
      "size": "large",
      "error_correction": "H",
      "colors": {
        "foreground": "#FF0000",
        "background": "#FFFFFF"
      },
      "format": "png"
    }
  }'

# Generate with query parameters (GET)
curl "http://localhost:3000/v2/generate?url=https://github.com&size=medium&foreground_color=%2300FF00"

# Download customized image
curl -X POST "http://localhost:3000/v2/image" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com",
    "customization": {
      "size": "medium",
      "format": "png"
    }
  }' -o custom_qr.png
```

### Using JavaScript/Fetch

```javascript
// V1 API - Basic QR code
const response = await fetch('http://localhost:3000/generate?url=https://github.com');
const data = await response.json();
document.getElementById('qr-image').src = data.qr_code;

// V2 API - Customized QR code
const customQR = await fetch('http://localhost:3000/v2/generate', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    url: 'https://github.com',
    customization: {
      size: 'large',
      error_correction: 'H',
      colors: {
        foreground: '#0066CC',
        background: '#FFFFFF'
      },
      format: 'png'
    }
  })
});
const customData = await customQR.json();
console.log('Custom QR:', customData);
```

### Using Python

```python
import requests

# Generate QR code
response = requests.get('http://localhost:3000/generate?url=https://github.com')
data = response.json()
print(data['qr_code'])  # Base64 encoded PNG

# Download PNG
response = requests.get('http://localhost:3000/image?url=https://github.com')
with open('qr_code.png', 'wb') as f:
    f.write(response.content)
```

## 🔧 Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server host address |
| `PORT` | `3000` | Server port |
| `LOG_LEVEL` | `info` | Logging level |
| `MAX_URL_LENGTH` | `2048` | Maximum URL length |

## 🏗️ Project Structure

```
qr-api/
├── src/
│   ├── config/          # Configuration management
│   ├── errors/          # Error handling
│   ├── handlers/        # API route handlers
│   ├── middleware/      # Request middleware
│   ├── models/          # Data models
│   ├── services/        # Business logic
│   └── main.rs         # Application entry point
├── Cargo.toml          # Rust dependencies
├── railway.toml        # Railway deployment configuration
└── README.md           # This file
```

## 🧪 Testing

### Manual Testing

1. **Health Check**
```bash
curl http://localhost:3000/
```

2. **Generate QR Code**
```bash
curl "http://localhost:3000/generate?url=https://example.com"
```

3. **Download PNG**
```bash
curl "http://localhost:3000/image?url=https://example.com" -o test.png
```

### Error Testing

```bash
# Empty URL
curl "http://localhost:3000/generate?url="

# URL too long
curl "http://localhost:3000/generate?url=$(printf 'a%.0s' {1..3000})"
```

## 📊 Performance

- **Response Time**: < 50ms for standard URLs
- **Memory Usage**: ~10MB base memory
- **Concurrent Requests**: Handles 1000+ concurrent requests
- **QR Code Quality**: High-quality PNG output

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) - Fast web framework
- QR code generation with [qrcode](https://github.com/kennytm/qrcode-rust) crate
- Image processing with [image](https://github.com/image-rs/image) crate

## 📞 Support

If you have any questions or need help, please open an issue on GitHub.

---

**Made with ❤️ in Rust**