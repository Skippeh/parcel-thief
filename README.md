# Parcel Server

## A custom server for Death Stranding Director's Cut

### Download

You can find the latest client and server version in [Releases](https://github.com/Skippeh/parcel-thief/releases).

### About

Parcel is a server for Death Stranding Director's Cut. It is designed for smaller groups of people who want to play and share their buildings and items without other players.

All buildings, missions, cargo (including lost cargo and shared lockers), and highway resources are synced between all players unlike the official server where they are semi-randomly distributed.

The server and client are both written in Rust.

### Game Installation

1. Download parcel-client from [Releases](https://github.com/Skippeh/parcel-thief/releases) and
   extract the files to the directory where Death Stranding DC is installed. If you're unsure where that is you can use these steps to find out:
   - Steam: Right click the game and click `Manage -> Browe local files`.
   - Epic: Right click the game and click `Manage` and then click the folder icon near the bottom next to the uninstall button.
2. Get the server url from the person who is hosting the server and then do **one** of the following:
   - Create a text file called `parcel-server-url` in the game directory and set it to the URL of the server.
   - Add a launch parameter in Steam or Epic for Death Stranding DC called `--parcel-server-url <url>` and set it to the URL of the server.
   - Set an environment variable called PARCEL_SERVER_URL to the URL of the server.
3. Launch parcel-client.exe and then launch the game from Steam or Epic.

There are also additional launch parameters that might help with troubleshooting.

#### Launch parameters

| Parameter           | Description                                             |
| ------------------- | ------------------------------------------------------- |
| --parcel-server-url | The URL of the server.                                  |
| --parcel-console    | Shows a console while the game is running with logging. |
| --parcel-debug      | Enable additional logging.                              |

Logs are also saved in the game directory to a file called `parcel-client.log`.

### Server Installation

1. Download parcel-server from [Releases](https://github.com/Skippeh/parcel-thief/releases) and
   extract the files anywhere. Preferrably not the game directory for tidyness because of some directories being created but it's up to you.
2. By default only a steam web api key is required to launch the server, but you might want to run the server with `--help` launch parameter to see what else you can configure. Note that the server is a commandline application so if you don't run it from an existing terminal the window will close after the process exits.
3. Optionally: configure the server by doing either of these:
   - Specify launch parameters directly when launching the server.
   - Create a .env file in the server directory to specify environment variables for the process.
4. Launch parcel-server.

#### PostgreSQL

The server uses PostgreSQL to store data.

If you don't have an existing server to use one will be automatically downloaded and configured, and start/stop with the server.

#### Linux dependencies

The linux server requires `libpq5` and glibc 2.31 or higher.
