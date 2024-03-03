# Rustinx

## Overview

Rustinx is a terminal-based monitoring tool designed specifically for Nginx servers. It provides real-time insights into Nginx's error logs, configuration status, and resource usage including CPU and memory. Built with Rust, leveraging crossterm for terminal graphics and tokio for asynchronous operations, Rustinx offers an efficient and user-friendly interface for server administrators.

## Features

- **Real-Time Error Log Monitoring**: Monitors the Nginx error log file and displays new log entries in real-time.
- **Nginx Configuration and Status**: Displays Nginx version, configuration test results, and PID.
- **Resource Usage**: Shows the current CPU and memory usage of Nginx.
- **User-Friendly Interface**: Utilizes a terminal-based UI for easy navigation and monitoring.

## Installation

Rustinx can be installed with the following command:

```bash
curl -sSL -o install_rustinx.sh https://raw.githubusercontent.com/charlesinwald/rustinx/master/install.sh && chmod +x install_rustinx.sh && ./install_rustinx.sh
```
## Usage
```bash
sudo rustinx
```
Quit with the `q` key.

Key Features:
 - Error Log Monitoring: Automatically starts monitoring the default Nginx error log located at /var/log/nginx/error.log. Custom log paths can be configured within the source code.
 - Configuration Test: Executes nginx -t to test the Nginx configuration files and displays the output.
 - Nginx Version and PID: Retrieves the Nginx version and current PID.
 - Resource Usage Information: Gathers and displays CPU and memory usage statistics for Nginx.
Requirements
 - Nginx: As a monitoring tool for Nginx, it's required to have Nginx installed on the server where Rustinx is deployed.
 - Root Access: Some features require root privileges to access Nginx's log files and process information.
## Contributing
Contributions to Rustinx are welcome! Whether it's adding new features, fixing bugs, or improving documentation, please feel free to fork the repository and submit a pull request.

## License
Rustinx is open-source software licensed under the MIT license.

## Disclaimer
Rustinx is a third-party tool and is not affiliated with or endorsed by the official Nginx project.