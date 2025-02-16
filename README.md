# Desk Agent
Local AI Desktop Assistant App

## Descrition
Desk Agent is offline, local AI Desktop Assistant application that is using Ollama server and have implemented few AI agents with tool to help user with everyday tasks. Desk Agent is crossplatform working on Windows, MacOS and Linux.

## Agents
- chat: simple chat agent
- web text scraper: scaping text from URL and sumarize in output
- images: searching images in local machine given the path or by description (vision model pulled into ollama required - **!WIP!**)

> [!NOTE]
> This project is in heavy development and more bugfixes, code cleaning/refactoring and more agents and tools implementation is needed

## Requirements
- Ollama installed on any machine in your local network
- In case Ollama server is running on another machine - Ollama need to be exposed (set 0.0.0.0:11434)
- Need to be able to pull models with tools & vision via terminal prompt

## Install/Build
> [!NOTE]
> Because of active development it's not published any release yet - this is tech demo for now. This is only a weekend project for now

- checkout desk_agent repository: `git clone https://github.com/Chleba/desk_agent.git`
- `cd ./desk_agennt`
- `cargo run` or `cargo build --release`
- in case of release build: run desk_agent in target/release folder

## TODO:
- [x] crossplatform UI & tools 
- [x] chat agent 
- [x] web scrape agent
- [x] image search agent
- [ ] image vision search imeplementation (very soon)
- [ ] web search agent
- [ ] pdf documents agent
- [ ] Ollama pulling models implementation
- [ ] layout design fixes & changes (to be more cleaner and simpler)
- [ ] much more ... sky(net) is a limit 
