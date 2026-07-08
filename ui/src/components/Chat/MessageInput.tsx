import React, { useState, KeyboardEvent } from 'react';
import { Button } from '../Common';

interface MessageInputProps {
  onSend: (content: string) => Promise<void>;
  disabled: boolean;
}

export default function MessageInput({ onSend, disabled }: MessageInputProps) {
  const [content, setContent] = useState('');
  const [isSending, setIsSending] = useState(false);

  const handleSend = async () => {
    if (!content.trim() || disabled) return;
    setIsSending(true);
    await onSend(content);
    setContent('');
    setIsSending(false);
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="flex space-x-2">
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={disabled ? 'Select an agent and start a conversation first' : 'Type your message...'}
        disabled={disabled}
        className="flex-1 px-3 py-2 border rounded-md resize-none"
        rows={3}
      />
      <Button onClick={handleSend} disabled={disabled || !content.trim() || isSending}>
        {isSending ? 'Sending...' : 'Send'}
      </Button>
    </div>
  );
}