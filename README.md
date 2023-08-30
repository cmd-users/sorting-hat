# Sorting Hat 🎩

<img align="right" width="256"  src="https://i.imgur.com/SGa9kKy.png" alt="sorting-hat logo"/>

🔰 Sorting Hat is a tool that empowers students to find their best suited learning methodology, backed up by extensive research and well known psycology concepts.

🧑‍💻 It uses the powers of predictive language models to interpret a series of open-ended answers so that all the relevant information is extracted to be used with traditional learning methodoly identification approaches.

📊 Sorting Hat can also be used by institutions to identify the profiles of each freshman student and organize them into classes that share a similar learning methodology to optimize teacher eficiency by using the best suited approach for that class.

<br>

# Table Of Contents
  * [Usage](#usage)
    * [No sign in student](#-no-sign-in-student)
    * [Student with result history enabled](#-student-with-result-history-enabled)
    * [Teacher, Institution or Organization](#-teacher,-Institution-or-Organization)
  * [Hosting](#hosting)
    * [1. Setup](#1-setup)
    * [2. Environment](#2-environment)
    * [3. Run](#3-run)
  * [License](#license)


<br>

## Usage

* ### 🤓 No sign in student:
To take the test once and immediately see your results:
1. Take the quiz.
2. See the result.

* ### 🧑‍🎓 Student with result history enabled:
To be able to keep track of your own history of results through time and compare results:
1. Sign in as a student to the Sorting Hat user database.
2. Take the quiz.
3. See the result.

* ### 👥 Teacher, Institution or Organization:
To be able to apply the quiz for multiple members of your hierarchy and have access to their results:
1. Sign in as a teacher to the Sorting Hat user database.
2. Share the invitation token with participants.
3. See the results.

<br>

## Hosting
To be able to host a Sorting Hat server you will need:
* [Rust](https://www.rust-lang.org/),
* [Diesel CLI](https://diesel.rs/guides/getting-started),
* [NPM](https://www.npmjs.com/),
* [Docker](https://www.docker.com/) with [docker-compose](https://docs.docker.com/compose/),
* and access to a [PostgreSQL database](https://www.postgresql.org/).

<br><h3 align="center">1. Setup</h3>

1. Sorting Hat requires the Nighly version of rust, so you need to install it if not present:
```command
rustup toolchain install nightly
```

2. In the project directory you'll need to run 
```command
rustup override set nightly
```

3. To install `diesel_cli` with postgres support:
```command
cargo install diesel_cli --no-default-features --features postgres
```

<br><h3 align="center">2. Environment</h3>

1. To be able to run Sorting Hat on your machine, Docker must be enabled or active.  

* **On windows:** this can be done by [ensuring that the Docker icon is present at your system tray](https://docs.docker.com/desktop/install/windows-install/#start-docker-desktop).

* **On Unix:** you can [start the Docker Daemon](https://docs.docker.com/config/daemon/start/) before running Sorting Hat with the command:

```command
sudo systemctl start docker
```

... or configure it to [always start at boot](https://docs.docker.com/engine/install/linux-postinstall/#configure-docker-to-start-on-boot-with-systemd) with:

```command
sudo systemctl enable docker.service
```

2. You will also need a PostgreSQL connection and provide the credentials to the program via the `.env` file.

<br><h3 align="center">3. Run</h3>

1. With everything set-up, run [`run.sh`](run.sh) (On Linux or MacOS) or [`run.bat`](run.bat) (On Windows) to start the server.
* Unix:
```command
./run.sh
```

* Windows:
```command
./run.bat
```

<br>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute the code as per the terms of the MIT License.

