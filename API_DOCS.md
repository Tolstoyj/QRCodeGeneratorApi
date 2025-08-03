# QR Code API Documentation v2.0.0

## Table of Contents
- [V1 API (Legacy)](#v1-api-legacy)
- [V2 API (Enhanced)](#v2-api-enhanced)
- [Error Handling](#error-handling)
- [Migration Guide](#migration-guide)

---

## V1 API (Legacy)

### GET /
**Description:** Health check endpoint  
**Response:** JSON with API status and available endpoints

### GET /generate
**Description:** Generate QR code as base64 PNG  
**Parameters:**
- `url` (required): The URL or text to encode

**Example:**
```bash
curl "http://localhost:3000/generate?url=https://example.com"
```

### GET /image
**Description:** Download QR code as PNG file  
**Parameters:**
- `url` (required): The URL or text to encode

**Example:**
```bash
curl "http://localhost:3000/image?url=https://example.com" -o qr.png
```

---

## V2 API (Enhanced)

### POST /v2/generate
**Description:** Generate customized QR code with full control  
**Content-Type:** application/json

**Request Body:**
```json
{
  "url": "string (required)",
  "customization": {
    "size": "small|medium|large|number (optional, default: medium)",
    "error_correction": "L|M|Q|H (optional, default: M)",
    "colors": {
      "foreground": "#RRGGBB (optional, default: #000000)",
      "background": "#RRGGBB (optional, default: #FFFFFF)"
    },
    "border_width": "number (optional, default: 4)",
    "format": "png|svg|jpeg (optional, default: png)"
  }
}
```

**Response:**
```json
{
  "qr_code": "data:image/format;base64,...",
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

**Examples:**
```bash
# Basic usage
curl -X POST "http://localhost:3000/v2/generate" \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Full customization
curl -X POST "http://localhost:3000/v2/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "customization": {
      "size": "large",
      "error_correction": "H",
      "colors": {
        "foreground": "#FF0000",
        "background": "#FFFFFF"
      },
      "border_width": 6,
      "format": "png"
    }
  }'
```

### GET /v2/generate
**Description:** Generate customized QR code using query parameters  
**Parameters:**
- `url` (required): The URL or text to encode
- `size` (optional): small, medium, large, or pixel value
- `error_correction` (optional): L, M, Q, or H
- `foreground_color` (optional): Hex color (URL encoded)
- `background_color` (optional): Hex color (URL encoded)
- `border_width` (optional): Number of pixels
- `format` (optional): png, svg, or jpeg

**Example:**
```bash
curl "http://localhost:3000/v2/generate?url=https://example.com&size=large&foreground_color=%23FF0000"
```

### POST /v2/image
**Description:** Download customized QR code as image file  
**Content-Type:** application/json

**Request Body:** Same as POST /v2/generate

**Response:** Binary image file with appropriate headers

**Example:**
```bash
curl -X POST "http://localhost:3000/v2/image" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "customization": {
      "size": "medium",
      "format": "png"
    }
  }' -o custom_qr.png
```

---

## Error Handling

### Error Response Format
```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

### Error Codes
- `VALIDATION_ERROR`: Invalid input parameters
- `GENERATION_ERROR`: QR code generation failed

### Common Validation Errors
- Empty URL
- URL too long (max 2048 characters)
- Invalid color format (must be #RRGGBB)
- Same foreground and background colors
- Insufficient color contrast (<3:1 ratio)
- Invalid size (custom size must be 50-2000px)
- Border width too large (max 50px)

---

## Migration Guide

### Migrating from V1 to V2

#### Minimal Changes (Keep existing behavior)
```bash
# V1
GET /generate?url=https://example.com

# V2 (equivalent)
GET /v2/generate?url=https://example.com
```

#### Take Advantage of New Features
```javascript
// V1 - Basic QR
const response = await fetch('/generate?url=https://example.com');

// V2 - Customized QR
const response = await fetch('/v2/generate', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    url: 'https://example.com',
    customization: {
      size: 'large',
      error_correction: 'H',
      colors: {
        foreground: '#0066CC',
        background: '#FFFFFF'
      }
    }
  })
});
```

#### Feature Comparison

| Feature | V1 | V2 |
|---------|----|----|
| **Basic QR Generation** | ✅ | ✅ |
| **Custom Sizes** | ❌ | ✅ |
| **Error Correction Levels** | ❌ | ✅ |
| **Color Customization** | ❌ | ✅ |
| **Multiple Formats** | ❌ | ✅ |
| **Border Control** | ❌ | ✅ |
| **JSON Response** | ✅ | ✅ |
| **Direct Download** | ✅ | ✅ |
| **Query Parameters** | ✅ | ✅ |
| **POST Support** | ❌ | ✅ |

#### Backward Compatibility Promise

✅ **V1 endpoints will never be removed**  
✅ **No breaking changes to V1 behavior**  
✅ **V1 continues to receive security updates**  
⚠️ **New features only added to V2+**

---

## 💻 Code Examples

### Client Libraries

#### JavaScript/TypeScript SDK

```typescript
class QRAPIClient {
  constructor(private baseURL: string = 'http://localhost:3000') {}

  async generateQR(url: string, options?: QROptions): Promise<QRResponse> {
    const response = await fetch(`${this.baseURL}/v2/generate`, {
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
    const response = await fetch(`${this.baseURL}/v2/image`, {
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

#### Python Client

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
            f"{self.base_url}/v2/generate",
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

#### cURL Examples

```bash
# Function for easy QR generation
generate_qr() {
  local url="$1"
  local size="${2:-medium}"
  local output="${3:-qr.png}"
  
  curl -s -X POST "http://localhost:3000/v2/image" \
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
A: Not currently, but planned for v2.1.0 (100 req/min)

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
A: Use high error correction (H) and overlay logo externally (planned native support in v3.0)

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
- [QR Code Best Practices](https://www.qrcode.com/en/about/standards.html)
- [API Status Page](https://status.deepplaystudio.com)
- [GitHub Repository](https://github.com/Tolstoyj/QRCodeGeneratorApi)

---

**Need help?** Open an issue on [GitHub](https://github.com/Tolstoyj/QRCodeGeneratorApi/issues) or contact devops@deepplaystudio.com