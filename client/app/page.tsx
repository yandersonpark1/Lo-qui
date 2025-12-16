"use client";

import { useEffect, useRef } from "react";
import Image from "next/image";
import messages from "./data"
import Messages from "./components/message";

export default function Home() {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView();
  }, []);

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
        const response = await fetch('http://localhost:8080/messages', {
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
        
        // Clear the content field after successful submission
        form.reset(); // Use the stored form reference instead of e.currentTarget
        
    } catch (error) {
        console.error('Fetch error:', error);
        alert(`Failed to send message: ${error}`);
    }
};

  return (
    <div className="flex flex-col min-h-screen text-white">
      <form onSubmit={handleSubmit}>
        <div className="fixed top-0 flex bg-black min-w-full z-10">
          <label>
            Name:
            <input type="text" name="username" className="border-white"/>
          </label>
        </div>
        <div className="flex flex-col m-5 ml-2 mr-2 mt-20 mb-24">
          {messages.map((v) => 
            <Messages key={v.id} data={v}/>
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