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

### Key Differences
1. **Customization Options**: V2 offers full control over QR appearance
2. **Error Correction**: V2 allows choosing error correction level
3. **Color Support**: V2 supports custom colors with contrast validation
4. **Multiple Formats**: V2 supports PNG, SVG, and JPEG
5. **Size Control**: V2 offers preset and custom sizes

### Backward Compatibility
- All V1 endpoints remain functional
- No breaking changes to existing integrations
- V1 endpoints will be maintained indefinitely

---

## Size Reference

| Size | Pixels | Use Case |
|------|---------|----------|
| small | 150×150 | Mobile apps, small displays |
| medium | 300×300 | Web pages, standard use |
| large | 600×600 | Print, high-resolution displays |
| custom | 50-2000 | Specific requirements |

## Error Correction Levels

| Level | Recovery | Use Case |
|-------|----------|----------|
| L | ~7% | Clean environments, maximum data |
| M | ~15% | General use (default) |
| Q | ~25% | Moderate damage expected |
| H | ~30% | Harsh environments, logos overlay |

---

*For more information, see the [README](README.md) and [ROADMAP](ROADMAP.md).*