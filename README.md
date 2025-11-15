# groupproject proposal 
For our final project, we are proposing a real time chat application. For this project, we will be implementing Rust into our backend and using React as our main framework for our frontend. Right now, we will initially start with a NoSQL relational database with MongoDB; however, as we work through the tests, we may switch to SQL for scalability. This project will highlight skills we have learned from this semester as well as understanding those skills on a lower level. These skills will include system-level programming, networking, concurrency, and system design just to name a few. For this initial project we hope to 

When working on this project, the first hurdle we will have to tackle is our system design for our full stack application. Our backend / server will have to listen to incoming client requests such as receiving and broadcasting messages ensuring that all connected clients in the chat simultaneously can interact. This will be the main component of our project, being built in Rust. We will have to connect our backend to both the frontend and our database either through websockets or APIs. While a database may not be needed for a small scale operation, we believe it will still be good to implement to store messages in the case users want to look back on any messages as well as in the case that a message fails through the network and needs to be retried. Furthermore, when scaling in the future, a database as proof of concept will enable easier setup. Our initial system design will use MongoDB as our database. MongoDB is a Non-SQL database storing information in json formats. We decided to start with MongoDB as most of the data stored will be messages which may be difficult to handle in relational SQL databases. We would also need to work on our client / frontend that will allow users to connect to the server, send messages, and display messages from other users. We will build our frontend using React (Javascript). While our group understands the basics of REST APIs such as get, post, and put; we will need to learn how to implement this in Rust syntax through frameworks integrated with websocket functionality. 

This real time chat application will allow users to send messages asynchronously with the server processing and forwarding messages without blocking other users. Users should be able to chat with minimal delay between sending and receiving messages. 

We will be implementing the backend /core of our project through Rust and Rust crates from crates.io. What we will have to learn is how to design a backend capable of handling multiple users concurrently while still delivering the functionality of an updated real time chat application. We believe this can be done by Rust’s Tokio crates which provide asynchronous programming. We will also be learning on a lower level about networks and how to manage them through websockets and how they establish connections from server to client side. We will also be using Axum as our primary web framework for any REST endpoint and Websocket routing. While these crates were the initial ones we found, and believe will be most helpful, we may end up changing this initial architecture to better suit our needs through trial and error. Like previously mentioned we will use React to create React components for our frontend to take messages from users as well gathering messages from other users to specific users. This will include a nice UI interface for users to view their messages. 

Our proposed schedule: 

Week 12: 
Setup Project with Rust using Cargo 
Review and learn about Tokio, Axum, and other rust crates that may help with async calls and websocket routing 
Examine overall Architecture
Week 13: 
Implement REST endpoints in Rust for message history and link to MongoDB / database 
Build initial websocket for single user to send message 
Status Update will be on 12/01 Monday of Week 14 at this point we should have initial users able to send messages to one another (not necessarily worried about concurrency)
Week 14: 
Extend server to handle multiple concurrent clients 
Make sure backend is capable of handling messages sent back and forth between users 
Week 15: 
Continue work for Week 14
Configure frontend to backend 
Exam Period / Reading Period: 
Any bug fixes
Add comments to code explaining 
Final Report 
Practice Presentation/Demo 
