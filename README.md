# 🏠 RentMap

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

A simple tool that scrapes rental listings, finds location coordinates, and reads text from images.

*Built with love using amazing open-source libraries! 🚀*

![Example usage](example.png)

## Prerequisites

Before getting started, make sure you have

- **Rust and Cargo** - [Install from rustup.rs](https://rustup.rs/)
- **Chromium-based browser** - Required for web scraping (Chrome, Edge, Brave, etc.)
- **Google API Key** - For geocoding and OCR features (see [Setup Your Google API Key](#setup-your-google-api-key))

> [!NOTE]
> The tool uses a headless browser for scraping rental websites, so a Chromium-based browser must be installed on your system.

## Quick Start

1. **Install the tool**
   ```bash
   cargo install --git https://github.com/Stanley5249/rentmap
   ```

2. **Try it out**
   ```bash
   rentmap --help
   ```

That's it! 🎉

## What You Can Do

**📋 Get Rental Listings**
- Extract data from rent.591.com.tw with smart page loading
- Save rental information as clean JSON files

**🗺️ Find Addresses**  
- Turn addresses into map coordinates using Google Maps
- Works with addresses in different languages and countries

**👁️ Read Text from Images**
- Extract text from photos using Google Vision API
- Supports multiple languages at once

**📄 Download Web Pages**
- Save and clean HTML content from websites
- Remove ads and scripts for cleaner data

## Examples

```bash
# Scrape rentals with custom output
rentmap lists "https://rent.591.com.tw/list?region=1&kind=2" --limit 3 --out-file my_rentals.json

# Geocode with language preference  
rentmap geocoding "東京駅" --language ja --region jp

# OCR with multiple language hints
rentmap ocr receipt.jpg --languages zh-Hant,en,ja

# Download and clean web pages
rentmap fetch "https://example.com" --out-dir downloads

# Need help with any command?
rentmap lists --help
```

## Setup Your Google API Key

> [!IMPORTANT]
> [Google Maps docs](https://developers.google.com/maps/documentation/geocoding/get-api-key) recommend API keys, but [Cloud Vision docs](https://cloud.google.com/vision/docs/setup) only mention service accounts. 
> 
> Good news - you can use **one API key** for both services. Perfect for personal use!

**Quick Setup** 

1. Go to [Google Cloud Console → APIs & Services](https://console.cloud.google.com/apis)
2. In the API Library, enable **Geocoding API** and **Cloud Vision API**
3. In Credentials, create an API key with access to both APIs

**Three ways to provide your API key**

### Environment variable
```bash
export GOOGLE_API_KEY=your-api-key
```

### Command line
```bash
rentmap geocoding "your-address" --api-key your-api-key
```

### Config file

See [Configuration](#configuration) section below for details.

## Configuration

Create `rentmap.toml` in current directory or home directory

```toml
# Google API configuration
api_key = "your-google-api-key"

# Geocoding settings
[geocoding]
language = "en"
region = "us"

# OCR settings
[ocr]
languages = ["zh-Hant", "en", "ja"]
```

> [!NOTE]
> **How it picks your settings** command line → environment variable → config file
> 
> **Where it looks for config** current directory → home directory
> 
> *Config file and all settings are optional.*

## Built With ❤️

RentMap stands on the shoulders of these incredible open-source projects

**🕷️ [spider](https://github.com/spider-rs/spider)** - Web crawling and scraping framework  
*Has potential and room for improvements, but gets the job done for our rental data extraction*

**🗺️ [google-maps](https://github.com/leontoeides/google_maps)** - Comprehensive Google Maps API client  
*A well-maintained community Rust crate for geocoding and mapping services*

**👁️ [google-cloud-vision-v1](https://github.com/googleapis/google-cloud-rust/tree/main/src/generated/cloud/vision/v1)** - Official Google Cloud Vision API client  
*Machine-generated bindings that work reliably for OCR and image text extraction*

**🛠️ Plus many other amazing Rust crates** including `scraper`, `clap`, `tokio`, `serde`, and more!

*Thank you to all the maintainers and contributors who make these tools possible!* 🙏

## License

This project is licensed under the MIT License.
