# 📚 QR Code API Documentation

> **Complete API reference for QR Code Generator Service**  
> Last Updated: August 2025

## 📖 Table of Contents

- [Quick Start](#-quick-start)
- [Authentication](#-authentication)
- [Base URL](#-base-url)
- [API Reference](#api-reference)
- [Data Types](#-data-types)
- [Error Handling](#-error-handling)
- [Rate Limiting](#-rate-limiting)
- [Code Examples](#-code-examples)
- [FAQ](#-faq)

## 🚀 Quick Start

### Generate Your First QR Code

```bash
# Simple QR generation with query parameters
curl "http://localhost:3000/generate?url=https://example.com"

# Customized QR generation with JSON body
curl -X POST "http://localhost:3000/generate" \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

## 🔐 Authentication

**Current Status**: No authentication required  
**Future**: API key authentication planned for future releases

## 🌐 Base URL

| Environment | URL |
|-------------|-----|
| Local Development | `http://localhost:3000` |
| Production | `https://api.yourdomain.com` |
| Staging | `https://staging-api.yourdomain.com` |

---

## API Reference

### 🏥 GET /

**Health Check Endpoint**

| Property | Value |
|----------|-------|
| **Purpose** | System health check and API information |
| **Authentication** | None |
| **Rate Limit** | Unlimited |

#### Response

```json
{
  "status": "healthy",
  "version": "2.0.0",
  "endpoints": [
    {"path": "/", "method": "GET", "description": "Health check"},
    {"path": "/generate", "method": "POST", "description": "Generate QR as JSON"},
    {"path": "/generate", "method": "GET", "description": "Generate QR via query"},
    {"path": "/image", "method": "POST", "description": "Download QR image"}
  ]
}
```

#### Example

```bash
curl -i http://localhost:3000/
```

---

### 📊 POST /generate

**Generate QR Code with Customization**

| Property | Value |
|----------|-------|
| **Purpose** | Generate QR with full customization options |
| **Method** | POST |
| **Content-Type** | `application/json` |
| **Max Payload** | 10KB |

#### Request Schema

```typescript
interface QRRequest {
  url: string;                    // Required: URL or text to encode
  customization?: {
    size?: 'small' | 'medium' | 'large' | number;  // Default: 'medium'
    error_correction?: 'L' | 'M' | 'Q' | 'H';      // Default: 'M'
    colors?: {
      foreground?: string;        // Hex color (default: '#000000')
      background?: string;        // Hex color (default: '#FFFFFF')
    };
    border_width?: number;        // 0-50 pixels (default: 4)
    format?: 'png' | 'svg' | 'jpeg';  // Default: 'png'
  };
}
```

#### Size Options

| Size | Pixels | Use Case |
|------|--------|----------|
| `small` | 150×150 | Thumbnails, mobile |
| `medium` | 300×300 | Web, standard use |
| `large` | 600×600 | Print, high-res |
| Custom | 50-2000 | Specific requirements |

#### Error Correction Levels

| Level | Recovery | Use Case |
|-------|----------|----------|
| `L` | ~7% | Maximum data capacity |
| `M` | ~15% | Balanced (default) |
| `Q` | ~25% | Moderate damage expected |
| `H` | ~30% | Logo overlay, harsh conditions |

#### Response Schema

```typescript
interface QRResponse {
  qr_code: string;           // Base64 encoded image data
  format: string;            // Output format used
  size: string;              // Size description
  error_correction: string;  // Error correction level used
  colors: {
    foreground: string;      // Foreground color used
    background: string;      // Background color used
  };
  border_width: number;      // Border width in pixels
}
```

#### Examples

```bash
# Minimal request
curl -X POST "http://localhost:3000/generate" \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Custom size and colors
curl -X POST "http://localhost:3000/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "customization": {
      "size": "large",
      "colors": {
        "foreground": "#0066CC",
        "background": "#F0F0F0"
      }
    }
  }'

# High error correction for logo overlay
curl -X POST "http://localhost:3000/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://brand.com",
    "customization": {
      "size": 500,
      "error_correction": "H",
      "border_width": 10
    }
  }'
```

---

### 🔗 GET /generate

**Generate QR via Query Parameters**

| Property | Value |
|----------|-------|
| **Purpose** | Generate QR using URL parameters |
| **Method** | GET |
| **Use Case** | Simple integrations, direct browser access |

#### Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `url` | string | Yes | - | URL or text to encode |
| `size` | string/number | No | `medium` | Size preset or pixels |
| `error_correction` | string | No | `M` | Error correction level |
| `foreground_color` | string | No | `#000000` | Hex color (URL encoded) |
| `background_color` | string | No | `#FFFFFF` | Hex color (URL encoded) |
| `border_width` | number | No | `4` | Border in pixels |
| `format` | string | No | `png` | Output format |

#### Examples

```bash
# Basic
curl "http://localhost:3000/generate?url=https://example.com"

# With size and colors
curl "http://localhost:3000/generate?url=https://example.com&size=large&foreground_color=%230066CC"

# Custom pixel size
curl "http://localhost:3000/generate?url=https://example.com&size=400"

# High error correction
curl "http://localhost:3000/generate?url=https://example.com&error_correction=H"

# Browser-friendly URL
http://localhost:3000/generate?url=https://github.com&size=large&foreground_color=%23FF0000
```

---

### 🖼️ POST /image

**Download Customized QR Image**

| Property | Value |
|----------|-------|
| **Purpose** | Generate and download customized QR image |
| **Method** | POST |
| **Request Type** | `application/json` |
| **Response Type** | `image/*` (based on format) |

#### Request

Same schema as [POST /generate](#-post-generate)

#### Response Headers

```http
Content-Type: image/png | image/svg+xml | image/jpeg
Content-Disposition: attachment; filename="qrcode-{size}x{size}.{ext}"
Content-Length: {size}
```

#### Examples

```bash
# Download PNG
curl -X POST "http://localhost:3000/image" \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}' \
  -o qr_code.png

# Download SVG
curl -X POST "http://localhost:3000/image" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "customization": {"format": "svg"}
  }' -o qr_code.svg

# Download with custom settings
curl -X POST "http://localhost:3000/image" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://brand.com",
    "customization": {
      "size": "large",
      "error_correction": "H",
      "colors": {
        "foreground": "#003366",
        "background": "#FFFFFF"
      },
      "format": "png"
    }
  }' -o brand_qr.png
```

---

## 📊 Data Types

### Color Format

| Format | Example | Description |
|--------|---------|-------------|
| Hex (6-digit) | `#FF0000` | Standard hex color |
| Hex (3-digit) | `#F00` | Short hex notation |

### Size Values

| Type | Valid Range | Examples |
|------|-------------|----------|
| Preset | `small`, `medium`, `large` | `"size": "large"` |
| Custom | 50-2000 | `"size": 450` |

### URL Format

| Type | Example | Notes |
|------|---------|-------|
| HTTP/HTTPS | `https://example.com` | Standard web URLs |
| Plain Text | `Hello World` | Any text string |
| Special Chars | `https://site.com?q=test` | URL encode in GET |

---

## 🚨 Error Handling

### Error Response Schema

```typescript
interface ErrorResponse {
  error: string;      // Human-readable error message
  code: string;       // Machine-readable error code
  details?: {         // Additional error context (when available)
    field?: string;   // Field that caused the error
    value?: any;      // Invalid value provided
    constraint?: string; // Validation constraint violated
  };
}
```

### Error Codes Reference

| Code | HTTP Status | Description | Example |
|------|-------------|-------------|---------|
| `VALIDATION_ERROR` | 400 | Invalid input parameters | Empty URL |
| `URL_TOO_LONG` | 400 | URL exceeds max length | > 2048 chars |
| `INVALID_COLOR` | 400 | Invalid color format | `#GGG` |
| `INVALID_SIZE` | 400 | Size out of range | `size: 3000` |
| `COLOR_CONTRAST` | 400 | Insufficient contrast | Same colors |
| `GENERATION_ERROR` | 500 | QR generation failed | Internal error |

### Common Error Scenarios

#### 1. Empty URL

```json
{
  "error": "URL cannot be empty",
  "code": "VALIDATION_ERROR",
  "details": {
    "field": "url",
    "constraint": "required"
  }
}
```

#### 2. Invalid Color Format

```json
{
  "error": "Invalid color format. Use #RRGGBB",
  "code": "INVALID_COLOR",
  "details": {
    "field": "foreground_color",
    "value": "red",
    "constraint": "hex_format"
  }
}
```

#### 3. Size Out of Range

```json
{
  "error": "Custom size must be between 50-2000 pixels",
  "code": "INVALID_SIZE",
  "details": {
    "field": "size",
    "value": 3000,
    "constraint": "range[50,2000]"
  }
}
```

### Validation Rules

| Field | Rule | Error Message |
|-------|------|---------------|
| `url` | Required, max 2048 chars | "URL cannot be empty" |
| `size` | 50-2000 for custom | "Size must be 50-2000px" |
| `colors` | Valid hex format | "Invalid color format" |
| `colors` | Sufficient contrast | "Colors too similar" |
| `border_width` | 0-50 pixels | "Border too large" |
| `format` | png/svg/jpeg | "Unknown format" |

---

## ⚡ Rate Limiting

**Current Status**: Not implemented  
**Planned**:
- Default: 100 requests/minute per IP
- Burst: 10 requests/second
- Headers: `X-RateLimit-*` will be added

---

## 💻 Code Examples

### JavaScript/TypeScript SDK

```typescript
class QRAPIClient {
  constructor(private baseURL: string = 'http://localhost:3000') {}

  async generateQR(url: string, options?: QROptions): Promise<QRResponse> {
    const response = await fetch(`${this.baseURL}/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url, customization: options })
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error);
    }
    
    return response.json();
  }

  async downloadQR(url: string, options?: QROptions): Promise<Blob> {
    const response = await fetch(`${this.baseURL}/image`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url, customization: options })
    });
    
    if (!response.ok) {
      throw new Error('Failed to generate QR code');
    }
    
    return response.blob();
  }
}

// Usage
const client = new QRAPIClient();
const qr = await client.generateQR('https://example.com', {
  size: 'large',
  error_correction: 'H'
});
```

### Python Client

```python
import requests
from typing import Optional, Dict, Any
import base64

class QRAPIClient:
    def __init__(self, base_url: str = "http://localhost:3000"):
        self.base_url = base_url
        self.session = requests.Session()
    
    def generate_qr(
        self,
        url: str,
        size: str = "medium",
        error_correction: str = "M",
        colors: Optional[Dict[str, str]] = None,
        format: str = "png"
    ) -> Dict[str, Any]:
        payload = {
            "url": url,
            "customization": {
                "size": size,
                "error_correction": error_correction,
                "format": format
            }
        }
        
        if colors:
            payload["customization"]["colors"] = colors
        
        response = self.session.post(
            f"{self.base_url}/generate",
            json=payload
        )
        response.raise_for_status()
        return response.json()
    
    def save_qr(self, url: str, filename: str, **kwargs):
        qr_data = self.generate_qr(url, **kwargs)
        # Extract base64 data
        image_data = qr_data['qr_code'].split(',')[1]
        # Decode and save
        with open(filename, 'wb') as f:
            f.write(base64.b64decode(image_data))

# Usage
client = QRAPIClient()
client.save_qr(
    "https://github.com",
    "github_qr.png",
    size="large",
    colors={"foreground": "#0366D6", "background": "#FFFFFF"}
)
```

### cURL Examples

```bash
# Function for easy QR generation
generate_qr() {
  local url="$1"
  local size="${2:-medium}"
  local output="${3:-qr.png}"
  
  curl -s -X POST "http://localhost:3000/image" \
    -H "Content-Type: application/json" \
    -d "{
      \"url\": \"$url\",
      \"customization\": {
        \"size\": \"$size\"
      }
    }" -o "$output"
  
  echo "QR code saved to $output"
}

# Usage
generate_qr "https://example.com" large my_qr.png
```

---

## ❓ FAQ

### General Questions

**Q: What's the maximum URL length?**  
A: 2048 characters

**Q: Can I generate QR codes for plain text?**  
A: Yes, any string can be encoded

**Q: Is there a rate limit?**  
A: Not currently, but planned (100 req/min)

**Q: Can I use this API in production?**  
A: Yes, it's production-ready with proper error handling

### Technical Questions

**Q: What's the difference between error correction levels?**  
A: Higher levels allow more damage but reduce data capacity:
- L: ~7% error correction (maximum data)
- M: ~15% error correction (balanced)
- Q: ~25% error correction (good for moderate damage)
- H: ~30% error correction (best for logos/harsh conditions)

**Q: Why use SVG format?**  
A: SVG is vector-based, infinitely scalable without quality loss

**Q: How do I embed a logo in the QR code?**  
A: Use high error correction (H) and overlay logo externally (planned native support)

**Q: What's the optimal QR code size?**  
A: Depends on use case:
- Screen display: 200-400px
- Print: 600px or higher
- Business cards: 150-200px

### Troubleshooting

**Q: Getting "Invalid color format" error?**  
A: Use 6-digit hex format with # prefix: `#FF0000`

**Q: QR code won't scan?**  
A: Check:
1. Sufficient contrast between colors
2. Adequate size for scanning distance
3. Appropriate error correction level

**Q: CORS errors in browser?**  
A: CORS is enabled by default. Check your request headers.

---

## 📚 Additional Resources

- [Main Documentation](README.md)
- [Development Roadmap](ROADMAP.md)
- [Architecture Guide](ARCHITECTURE.md)
- [Contributing Guide](CONTRIBUTING.md)
- [QR Code Best Practices](https://www.qrcode.com/en/about/standards.html)
- [GitHub Repository](https://github.com/Tolstoyj/QRCodeGeneratorApi)

---

**Need help?** Open an issue on [GitHub](https://github.com/Tolstoyj/QRCodeGeneratorApi/issues) or contact devops@deepplaystudio.com