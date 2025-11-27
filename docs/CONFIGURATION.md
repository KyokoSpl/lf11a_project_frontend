# Configuration Guide

## Environment Variables

The application supports configuration through a `.env` file. Copy `.env.example` to `.env` and modify the values as needed.

### Configuration Options

#### API Configuration
```bash
# Base URL of the backend API
API_BASE_URL=http://loaclhost:8082

# API prefix (usually /api)
API_PREFIX=/api
```

#### API Routes
```bash
# API endpoint routes
ROUTE_DEPARTMENTS=/departments
ROUTE_EMPLOYEES=/employees
ROUTE_SALARY_GRADES=/salary-grades
```

#### Logging Configuration
```bash
# Log level: TRACE, DEBUG, INFO, WARN, ERROR
LOG_LEVEL=INFO

# Enable/disable debug mode
DEBUG_MODE=false
```

#### Application Configuration
```bash
# Application metadata
APP_NAME=LF11A Project Frontend
APP_VERSION=0.1.0
```

#### Network Configuration
```bash
# Request timeout in seconds
REQUEST_TIMEOUT_SECS=30

# Number of retry attempts for failed requests
RETRY_ATTEMPTS=3
```

## Usage

1. Copy the example configuration:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your settings:
   ```bash
   nano .env  # or use your preferred editor
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## Logging

The application includes a configurable logging system controlled by the `LOG_LEVEL` environment variable:

- **TRACE**: Most verbose, logs everything
- **DEBUG**: Detailed debugging information
- **INFO**: General informational messages (default)
- **WARN**: Warning messages
- **ERROR**: Error messages only

### Using Logging in Code

```rust
use crate::{log_debug, log_info, log_warn, log_error};

log_debug!("This is a debug message: {}", value);
log_info!("Application started");
log_warn!("Warning: {}", warning_msg);
log_error!("Error occurred: {}", error);
```

## Development vs Production

For development, you might want:
```bash
LOG_LEVEL=DEBUG
DEBUG_MODE=true
API_BASE_URL=http://localhost:8082
```

For production:
```bash
LOG_LEVEL=WARN
DEBUG_MODE=false
API_BASE_URL=http://your-production-server:8082
```

## Security Note

**Important**: Never commit your `.env` file to version control. It's already included in `.gitignore` to prevent accidental commits.
