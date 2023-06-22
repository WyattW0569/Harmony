*Harmony*

**Main Goal**
-> Build a Functional web app in only rust

**Basic Overview**
-> Rust Programing language

-> Frameworks used:
---> Actix Web 3.2.0
------> Backend Development
---> Yew 0.20.0
------> Frontend Development

-> NO MANUALLY TYPED JAVA SCRIPT!!!

**Development Timeline (ROUGH)**
-> April
---> Discuss Idea for app
---> Draw a super hyper realistic sketch in MS Paint
---> Pick out frameworks
---> Create Project file structure and set up github

-> May
---> Got a basic Actix Web server running
---> Hot reloading
---> Simple yew frontend setup
---> Work on getting the backend and frontend to talk to eachother (This took ages)
---> Created our own custom endpoints
---> Tom learned a bunch about Websockets, which are an integral part of the project 
---> User Input (HUGE!)

-> June
---> Created frontend websocket logic
---> Linked input from frontend to backend
---> Very barebones chat app finished
---> Began complete redesign of the frontend
---> Images properly displaying to frontend
---> Implemented custom commands for the backend (Whisper, Help, ect.)
---> HUGE BACKEND BUG FIXES
---> Custom nicknames added
---> Minor Bug fixes
---> WRITING THIS

**Known Issues**
-> Name and Room components don't hot refresh on route change, requires manual refresh
-> ~~Formatting error for system commands
-> Pipe operator ends message
-> Messages don't wrap
-> 16 user limit is purely cosmetic