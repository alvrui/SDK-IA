import React from 'react';

export default function Navbar() {
  return (
    <nav className="bg-blue-600 text-white px-6 py-3 shadow-md">
      <div className="flex justify-between items-center max-w-7xl mx-auto">
        <div className="text-xl font-bold">SDK-IA Unified</div>
        <div className="flex space-x-4">
          <a href="/" className="hover:text-blue-200">Projects</a>
          <a href="/agents" className="hover:text-blue-200">Agents</a>
          <a href="/chat" className="hover:text-blue-200">Chat</a>
          <a href="/generate" className="hover:text-blue-200">Generate</a>
        </div>
      </div>
    </nav>
  );
}