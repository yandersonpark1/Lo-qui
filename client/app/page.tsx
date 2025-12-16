"use client";

import { useEffect, useRef, useState } from "react";
import Messages from "./components/message";

interface Message {
  id: number;
  username: string;
  content: string;
  created_at: string;
}

export default function Home() {
  const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL;
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [username, setUsername] = useState<string>(""); // Store username in state

  // Fetch messages on component mount
  useEffect(() => {
    fetchMessages();
  }, []);

  // Scroll to bottom when messages change
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const fetchMessages = async () => {
    try {
      const response = await fetch(`${BACKEND_URL}/messages`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      // Reverse the messages so newest is at bottom
      setMessages(data.messages.reverse());
    } catch (error) {
      console.error('Error fetching messages:', error);
    }
  };

  // POST request
  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    
    const form = e.currentTarget;
    const formData = new FormData(form);
    const formUsername = formData.get("username") as string;
    const formContent = formData.get("content") as string;
    
    if(!formUsername || !formContent || formUsername.trim() === '' || formContent.trim() === ''){
      alert('fill in all necessary inputs');
      return;
    }
    
    try {
        const response = await fetch(`${BACKEND_URL}/messages`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                content: formContent,
                username: formUsername 
            })
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        console.log('Success:', data);
        
        setUsername(formUsername);
        const contentInput = form.querySelector('input[name="content"]') as HTMLInputElement;
        if (contentInput) {
          contentInput.value = '';
        }
        
        fetchMessages();
        
    } catch (error) {
        console.error('Fetch error:', error);
        alert(`Failed to send message: ${error}`);
    }
  };

  return (
    <div className="flex flex-col min-h-screen text-white">
      <form onSubmit={handleSubmit}>
        <div className="fixed top-0 flex justify-between bg-black min-w-full z-10">
          <label>
            Name:
            <input 
              type="text" 
              name="username" 
              className="border-white"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
          </label>
          <div onClick={fetchMessages} className="m-2 px-2 py-1 border-2 border-white hover:bg-white hover:text-black transition-colors">
            Reload
          </div>
        </div>
        <div className="flex flex-col m-5 ml-2 mr-2 mt-20 mb-24">
          {messages.map((v) => 
            <Messages key={v.id} data={{
              id: v.id,
              username: v.username,
              content: v.content,
              timestamp: v.created_at
            }}/>
          )}
          <div ref={messagesEndRef} />
        </div>
        <div className="flex justify-center fixed bottom-0 left-0 right-0 bg-black gap-2">
          <input type="text" name="content" className="m-5 w-[90%] border-2 border-white"/>
          <button type="submit" className="m-5 px-4 py-2 border-2 border-white hover:bg-white hover:text-black transition-colors">
            Send
          </button>
        </div>
      </form>
    </div>
  );
}