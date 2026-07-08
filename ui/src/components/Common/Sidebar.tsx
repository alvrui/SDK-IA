import React from 'react';

export default function Sidebar() {
  return (
    <aside className="w-64 bg-gray-800 text-white min-h-screen">
      <div className="p-4">
        <h2 className="text-lg font-semibold mb-4">Navigation</h2>
        <ul className="space-y-2">
          <li><a href="/" className="block px-4 py-2 hover:bg-gray-700 rounded">Projects</a></li>
          <li><a href="/agents" className="block px-4 py-2 hover:bg-gray-700 rounded">Agents</a></li>
          <li><a href="/chat" className="block px-4 py-2 hover:bg-gray-700 rounded">Chat Interface</a></li>
          <li><a href="/generate" className="block px-4 py-2 hover:bg-gray-700 rounded">Generation</a></li>
        </ul>
      </div>
    </aside>
  );
}