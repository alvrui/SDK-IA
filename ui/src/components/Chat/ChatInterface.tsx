import React, { useState, useEffect } from 'react';
import { useAgents, useConversations, useChat } from '../../hooks';
import { LoadingSpinner, ErrorDisplay } from '../Common';
import MessageList from './MessageList';
import MessageInput from './MessageInput';

export default function ChatInterface() {
  const { agents, loading: agentsLoading, error: agentsError } = useAgents();
  const [selectedAgentId, setSelectedAgentId] = useState<string>('');
  const [selectedProjectId, setSelectedProjectId] = useState<string>('');
  const [conversationTitle, setConversationTitle] = useState<string>('New Conversation');
  const { conversations, loading: convLoading, createConversation } = useConversations(selectedAgentId, selectedProjectId);
  const [activeConversationId, setActiveConversationId] = useState<string>('');
  const { messages, loading: msgLoading, error: msgError, sendMessage, fetchMessages } = useChat(activeConversationId);

  useEffect(() => {
    if (agents.length > 0 && !selectedAgentId) {
      setSelectedAgentId(agents[0].id);
    }
  }, [agents, selectedAgentId]);

  const handleStartConversation = async () => {
    if (!selectedAgentId) return;
    const conversation = await createConversation(selectedAgentId, selectedProjectId, conversationTitle);
    if (conversation) {
      setActiveConversationId(conversation.id);
    }
  };

  const handleSendMessage = async (content: string) => {
    if (!activeConversationId) {
      await handleStartConversation();
      if (!activeConversationId) return;
    }
    await sendMessage(activeConversationId, content);
  };

  if (agentsLoading) return <LoadingSpinner text="Loading agents..." />;
  if (agentsError) return <ErrorDisplay error={agentsError} />;

  return (
    <div className="flex flex-col h-full max-h-[80vh]">
      <div className="flex space-x-4 mb-4">
        <select
          value={selectedAgentId}
          onChange={(e) => setSelectedAgentId(e.target.value)}
          className="px-3 py-2 border rounded-md"
        >
          {agents.map(agent => (
            <option key={agent.id} value={agent.id}>{agent.name} ({agent.model})</option>
          ))}
        </select>
        <input
          type="text"
          value={conversationTitle}
          onChange={(e) => setConversationTitle(e.target.value)}
          placeholder="Conversation title"
          className="flex-1 px-3 py-2 border rounded-md"
        />
        <button onClick={handleStartConversation} className="px-4 py-2 bg-blue-600 text-white rounded-md">
          Start Chat
        </button>
      </div>

      {msgLoading && !messages.length ? (
        <LoadingSpinner text="Loading messages..." />
      ) : (
        <>
          <MessageList messages={messages} loading={msgLoading} />
          <MessageInput onSend={handleSendMessage} disabled={!activeConversationId} />
        </>
      )}
      {msgError && <ErrorDisplay error={msgError} />}
    </div>
  );
}