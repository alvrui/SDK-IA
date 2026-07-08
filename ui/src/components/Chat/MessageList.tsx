import React from 'react';
import { ChatMessage } from '../../hooks/useChat';
import { LoadingSpinner } from '../Common';

interface MessageListProps {
  messages: ChatMessage[];
  loading: boolean;
}

export default function MessageList({ messages, loading }: MessageListProps) {
  return (
    <div className="flex-1 overflow-y-auto border p-4 rounded-md mb-4 bg-gray-50">
      {messages.length === 0 ? (
        <p className="text-gray-500 text-center">No messages yet. Start a conversation!</p>
      ) : (
        <div className="space-y-4">
          {messages.map(message => (
            <div
              key={message.id}
              className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-xs lg:max-w-md px-4 py-2 rounded-lg ${message.role === 'user' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-800'}`}
              >
                <p className="whitespace-pre-wrap">{message.content}</p>
                <span className={`text-xs mt-1 block ${message.role === 'user' ? 'text-blue-100' : 'text-gray-500'}`}>
                  {new Date(message.timestamp).toLocaleTimeString()}
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
      {loading && <LoadingSpinner size="sm" />}
    </div>
  );
}