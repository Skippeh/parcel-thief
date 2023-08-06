# Parcel - A Private Server for Death Stranding Director's Cut

> Parcel is a server for Death Stranding Director's Cut. It is designed for smaller groups of people who want to play and share their buildings and items without other players.
>
> All buildings, missions, cargo (including lost cargo and shared lockers), and highway resources are synced between all players unlike the official server where they are semi-randomly distributed.

## Download

You can find the latest client and server version in [Releases](https://github.com/Skippeh/parcel-thief/releases).

Note that the project is still in development by a single developer so there will be bugs and unexpected behaviour.

## Game Installation

**NOTE: Only Director's Cut is supported. Original Death Stranding is untested and is very unlikely to work.**

1. Download parcel-client from [Releases](https://github.com/Skippeh/parcel-thief/releases) and
   extract the files to the directory where Death Stranding DC is installed. If you're unsure where that is you can use these steps to find out:
   - Steam: Right click the game and click `Manage -> Browe local files`.
   - Epic: Right click the game and click `Manage` and then click the folder icon near the bottom next to the uninstall button.
2. Get the server url from the person who is hosting the server and then do **one** of the following:
   - Create a text file called `parcel-server-url` in the game directory and set it to the URL of the server.
   - Add a launch parameter in Steam or Epic for Death Stranding DC called `--parcel-server-url <url>` and set it to the URL of the server. (NOTE: this seems to be broken on Epic)
   - Set an environment variable called PARCEL_SERVER_URL to the URL of the server.
3. Launch parcel-client.exe and wait for the game to launch.

There are also additional launch parameters that might help with troubleshooting.

### Launch parameters

| Parameter                   | Description                                             |
| --------------------------- | ------------------------------------------------------- |
| --parcel-server-url `<url>` | The URL of the server.                                  |
| --parcel-console            | Shows a console while the game is running with logging. |
| --parcel-debug              | Enable additional logging.                              |

Logs are also saved in the game directory to a file called `parcel-client.log`.

## Server Installation

1. Download parcel-server from [Releases](https://github.com/Skippeh/parcel-thief/releases) and
   extract the files anywhere. Preferrably not the game directory for tidyness because of some directories being created but it's up to you.
2. By default only a steam web api key is required to launch the server, but you might want to run the server with `--help` launch parameter to see what else you can configure. Note that the server is a commandline application so if you don't run it from an existing terminal the window will close after the process exits.
   - Optionally: configure the server by doing either of these:
     - Specify launch parameters directly when launching the server.
     - Create a .env file in the server directory to specify environment variables for the process.
3. Launch parcel-server.
4. Optionally, configure the whitelist and other runtime configurable parameters from the frontend.
   You can access it from the web browser at `/frontend`.
   A default admin account is created and logged on startup if an account without the `Manage accounts` permission exists.
   You can also edit the whitelist and server config file directly in the server's `data` directory. Restarting the server after editing is not required.

### PostgreSQL

The server uses PostgreSQL to store data.

If you don't have an existing server to use one will be automatically downloaded and configured, and start/stop with the server.

**There is an issue with this on some Windows configurations where graceful shutdown doesn't work and corrupts the database. I recommend testing to see if ctrl+c shuts down the server correctly before using it.**

### Linux dependencies

The linux server requires `libpq5` and `glibc-2.31` or higher.

## Terminology

Figured out from reverse engineering, the game (and thus also this project) refers to the following terms:

- Mission: Lost cargo (either dropped in the world or in a shared locker), catapulted cargo containers, and supply requests.
- Wasted baggage: Broken or fully used items (such as ammo magazines, grenades, or bloodpacks), that have been abandoned in the world.
- Road: A path created by a player walking.
- Highway: The repairable roads that go between distribution centers and cities.
- Qpid object: Any vehicle (trucks, bikes, etc) or object that can be built, such as ladders, postboxes, bridges, mushrooms (from peeing), or rock formations (from resting).
- Qpid id: The id of an area in the game that is "owned" by a waystation or distribution center/city.
- Area id/hash: The id of one of the following: Western region, Central region, and Eastern region. There is also one for the intro region, but it's unused in the server due to there not being any online activity there.

Note that this list might be inconclusive and maybe even slightly incorrect.

## TODOs

### Server

- [ ] More testing with more players doing the same missions, etc. Generally I've only been able to test with two accounts.
- [ ] Account id's are some sort of hash that needs to be figured out, otherwise some non-critical things don't work properly (listed in known issues).

### Client

- [ ] Remove hard coded address offset for auth url in client and search for it instead.

## Less important TODOs

### Server

- [ ] Add all remaining object types and name them if possible. There are probably a lot more of them that I haven't encountered yet. [Known object types can be found here](https://github.com/Skippeh/parcel-thief/blob/main/parcel-common/src/api_types/object.rs#L155).
- [x] Figure out hashes for cargo items.
- [x] Figure out the qpid id for each area in the game.
- [ ] Figure out hashes for static locations such as pre-placed post boxes.
- [ ] If there's a want for it, implement ranked missions and rewards. The missions seem to be hard coded in the game and the server simply refers to a group of them by season id or something, but it could still be fun.
- [ ] Implement the following endpoints if necessary (not sure if they're used):
  - [ ] deleteHighwayResources (there's no way to delete highway segments/resources in game)
  - [ ] deletePlayerRankingRecords (don't think there's a way to delete ranking records in game)
  - [ ] deleteRoads (don't think there's ever a scenario where player paths are deleted)

## Ideas

- Add some sort of mission creator to the frontend which puts lost cargo in the world, or shared lockers etc.

## Known issues

- Various missing object types, usually noticed when the game calls the `findQpidObjects` or `createObject` endpoints with unknown object types in the request. It will still work but a warning will be logged.
- Game does not query player profiles which leads to not being able to see stats for other players (total likes for example), most likely due to account id not being a valid hash of some sort.
- Game does not show avatars of other players, most likely due to account id not being a valid hash of some sort.
- Embedded PostgreSQL database doesn't gracefully shutdown on some Windows installs. Not sure why. I suspect it might have something to do with the `pg-embed` crate or a permission issue as replicating the shutdown command with pg_ctl seems to work fine.

## Building

To build `parcel-server` you need to set `PQ_LIB_DIR` to the directory of the PostgreSQL 15 lib directory. You can find an example of this by looking at the github action workflow that builds and creates a release.

To build `parcel-client` and `client-injector` you need to use Rust nightly.

After the above is done, all you need to do is run `cargo build`.

## Acknowledgements

Thanks to:

- [Decima Workshop](https://github.com/ShadelessFox/decima) by ShadelessFox for figuring out game data layout and making it very easy to browse through the game's data.
