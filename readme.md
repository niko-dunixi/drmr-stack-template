# DRMR Stack Template

Pronounced like Dreamer ([dree·mr](https://www.google.com/search?q=how+to+pronounce+dreamer))

**D**ocker<br>
**R**ust<br>
**M**ongo<br>
**R**eact<br>

## Quick Start
If you are viewing this project on GitHub, select the ["Use this Template"](https://github.com/paul-nelson-baker/drmr-stack-template/generate) button.

Once you've cloned your new project locally, you can use `make` to start up your project

| Command | Description |
| - | - |
| `make up` | Will start the docker compose environment, mapping port 80 to your localhost under the `www.drmr.localhost` domain. |
| `make down` | Will tear down your docker compose environment. |
| `make clean` | Will remove containers and clean up Docker volumes. Implies `make down`. |

You can visit [http://www.drmr.localhost/](http://www.drmr.localhost/) once you've started your stack and you'll be greeted by React running in development mode behind Traefik. Traefik also binds the backend to [http://www.drmr.localhost/api](http://www.drmr.localhost/api). You can make rest calls to your same host (no need to change port.

You can view the Traefik Dashboard by browsing to [http://localhost:8080](http://localhost:8080).

## The what and why
One of the things I do in my personal workflow is take a Docker-first approach. I like consistency when I jump between machines so I don't have to think about my different configurations.

One thing I found, talking to one of my peers, is that they perfer the quickest and dirtiest path forward when working on a proof of concept.

There the most commonly known stacks such as:
| Acronym | Components |
| --- | --- |
| LAMP | <ul><li>Linux</li><li>Apache</li><li>MySQL</li><li>PHP</li></ul> |
| MEAN | <ul><li>Mongo</li><li>Express</li><li>Angular</li><li>Node</li></ul> |
| MERN | <ul><li>Mongo</li><li>Express</li><li>React</li><li>Node</li></ul> |
| WISA | <ul><li>Windows Server</li><li>IIS</li><li>Microsoft SQL Server</li><li>ASP.NET</li></ul> |

LAMP and WISA are both tied to a specific operating system as well as a specific serving option. In the MEAN and MERN solution, your operating system is agnostic, but both your backend and serving option are Express, which are both dependant on Node.

DRMR is a different approach. By using Docker, you don't need to care about the operating system or serving option. I picked Rust arbitrarily to help build the acronym. You could just as easily replace it with Go, Express, Python or anything else. Same thing with Mongo, you could easily replace this with MySQL or Postgres as long as you build with Domain Driven Design this is an arbitrary component as well.

You're not even tied to Docker. You could use
* [Rancher Desktop](https://rancherdesktop.io/) with [Nerdctl](https://github.com/containerd/nerdctl)
* [Podman](https://podman.io/)/[Podman-Compose](https://github.com/containers/podman-compose)

This is ultimately the point. To have a template that you can start with that is just as general and agnoistic as the cloud-platform you'll probably want to deploy to eventually without tying your hands early on.
