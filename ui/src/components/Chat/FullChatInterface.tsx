---
import React, { useState, useEffect, useRef } from 'react';
import { Agent } from '../../types/agent';
import { sendAgentMessage, listAgents } from '../../api/agents';
import './FullChatInterface.css';

interface Message {
  id: string;
  role: 'user' | 'agent' | 'system';
  content: string;
  timestamp: string;
  conversation_id?: string;
}

interface FullChatInterfaceProps {
  initialAgentId?: string;
}

export default function FullChatInterface({ initialAgentId }: FullChatInterfaceProps) {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [messageInput, setMessageInput] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [conversationId, setConversationId] = useState('');
  const [forceNewConversation, setForceNewConversation] = useState(false);
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [status, setStatus] = useState('Idle');
  const [isSending, setIsSending] = useState(false);

  const ws = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Cargar agentes
  useEffect(() => {
    const loadAgents = async () => {
      try {
        const response = await listAgents();
        setAgents(response.agents);
        if (initialAgentId) {
          const initialAgent = response.agents.find(a => a.id === initialAgentId);
          if (initialAgent) setSelectedAgent(initialAgent);
        } else if (response.agents.length > 0) {
          setSelectedAgent(response.agents[0]);
        }
      } catch (error) {
        console.error('Error loading agents:', error);
      }
    };
    loadAgents();
  }, [initialAgentId]);

  // Conectar WebSocket
  useEffect(() => {
    if (!selectedAgent) return;

    // Desconectar WebSocket anterior si existe
    if (ws.current) {
      ws.current.close();
    }

    const wsUrl = `ws://${window.location.hostname}:9000/ws`;
    ws.current = new WebSocket(wsUrl);

    ws.current.onopen = () => {
      setIsConnected(true);
      setStatus('Conectado');
    };

    ws.current.onclose = () => {
      setIsConnected(false);
      setStatus('Desconectado');
    };

    ws.current.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'response') {
        const newMessage: Message = {
          id: `msg-${Date.now()}`,
          role: 'agent',
          content: data.content,
          timestamp: data.timestamp,
          conversation_id: data.conversation_id
        };
        setMessages(prev => [...prev, newMessage]);
        if (data.conversation_id && !conversationId) {
          setConversationId(data.conversation_id);
        }
      }
    };

    ws.current.onerror = (error) => {
      console.error('WebSocket error:', error);
      setStatus('Error de conexión');
    };

    return () => {
      if (ws.current) {
        ws.current.close();
      }
    };
  }, [selectedAgent, conversationId]);

  // Auto-scroll a los mensajes
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!selectedAgent || !messageInput.trim() || !ws.current) return;

    setIsSending(true);
    const newMessage: Message = {
      id: `msg-${Date.now()}`,
      role: 'user',
      content: messageInput,
      timestamp: new Date().toLocaleTimeString('es-ES', {
        hour: '2-digit',
        minute: '2-digit'
      })
    };

    setMessages(prev => [...prev, newMessage]);

    try {
      // Enviar mensaje vía WebSocket
      ws.current.send(JSON.stringify({
        type: 'message',
        agent_id: selectedAgent.id,
        conversation_id: forceNewConversation ? null : conversationId,
        content: messageInput
      }));

      setMessageInput('');
      if (forceNewConversation) {
        setConversationId('');
        setForceNewConversation(false);
      }
    } catch (error) {
      console.error('Error sending message:', error);
      setStatus('Error al enviar');
    } finally {
      setIsSending(false);
    }
  };

  const handleNewConversation = () => {
    setMessages([]);
    setConversationId('');
    setForceNewConversation(true);
    setStatus('Nueva conversación');
  };

  const handleCopyAgentId = () => {
    if (selectedAgent) {
      navigator.clipboard.writeText(selectedAgent.id);
      setStatus('ID copiado al portapapeles');
      setTimeout(() => setStatus('Idle'), 2000);
    }
  };

  const toggleTheme = () => {
    const newTheme = theme === 'dark' ? 'light' : 'dark';
    setTheme(newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  };

  const handleRefresh = async () => {
    if (selectedAgent) {
      try {
        const response = await listAgents();
        setAgents(response.agents);
        setStatus('Agentes actualizados');
        setTimeout(() => setStatus('Idle'), 2000);
      } catch (error) {
        setStatus('Error al refrescar');
        setTimeout(() => setStatus('Idle'), 2000);
      }
    }
  };

  return (
    <div className="app" data-theme={theme}>
      <header className="topbar">
        <div className="brand">
          <div className="brand-mark">S</div>
          <div className="brand-text">
            <h1>Secretario de Agentes</h1>
            <p>Centralita operativa para agentes Mistral</p>
          </div>
        </div>
        <div className="toolbar">
          <button className="button ghost" onClick={toggleTheme} type="button">
            Cambiar tema
          </button>
          <button className="button" onClick={handleRefresh} type="button">
            Refrescar
          </button>
          <span className={`badge ${isConnected ? 'success' : ''}`}>
            {isConnected ? 'Conectado' : 'Desconectado'}
          </span>
        </div>
      </header>

      <aside className="panel sidebar">
        <div className="panel-header">
          <div>
            <h2>Agentes</h2>
            <p>Selecciona, filtra y administra tu pool</p>
          </div>
          <span className="badge">{agents.length}</span>
        </div>
        <div className="sidebar-body">
          <input
            className="search"
            type="search"
            placeholder="Buscar agente..."
            onChange={(e) => {
              // Implementar filtro más adelante
            }}
          />
          <div className="agent-list">
            {agents.length === 0 ? (
              <div className="empty">No hay agentes disponibles</div>
            ) : (
              agents.map(agent => (
                <button
                  key={agent.id}
                  className={`agent-card ${selectedAgent?.id === agent.id ? 'active' : ''}`}
                  onClick={() => setSelectedAgent(agent)}
                  type="button"
                >
                  <div className="agent-title">
                    <span className="agent-name">{agent.name}</span>
                    <span className="badge success">activo</span>
                  </div>
                  <div className="agent-meta">
                    <div className="mono">{agent.id}</div>
                    <div>
                      Conversación: <span className="mono">
                        {conversationId || 'nueva'}
                      </span>
                    </div>
                  </div>
                </button>
              ))
            )}
          </div>
        </div>
      </aside>

      <main className="panel chat">
        <div className="panel-header">
          <div>
            <h2>{selectedAgent?.name || 'Sin agente seleccionado'}</h2>
            <p>
              {selectedAgent
                ? `Conversation ID activa: ${conversationId || 'nueva conversación'}`
                : 'Selecciona un agente para trabajar'
              }
            </p>
          </div>
          <div className="toolbar-group">
            <button className="button" onClick={handleNewConversation} type="button">
              Nueva conversación
            </button>
            <button className="button" onClick={handleCopyAgentId} type="button">
              Copiar ID
            </button>
          </div>
        </div>
        <div className="chat-shell">
          <div className="chat-meta">
            {selectedAgent && (
              <>
                <span className="badge">
                  Agente <span className="mono">{selectedAgent.id}</span>
                </span>
                <span className="badge">
                  Conversación <span className="mono">{conversationId || 'nueva'}</span>
                </span>
                <span className="badge">Mensajes {messages.length}</span>
              </>
            )}
          </div>
          <div className="chat-body">
            {messages.length === 0 ? (
              <div className="empty">
                {selectedAgent
                  ? 'Aún no hay mensajes para este agente.'
                  : 'No hay conversación cargada todavía.'
                }
              </div>
            ) : (
              messages.map(message => (
                <div
                  key={message.id}
                  className={`message ${message.role}`}
                >
                  <div className="message-head">
                    <span>{message.role}</span>
                    <span>{message.timestamp}</span>
                  </div>
                  <div className="message-content">{message.content}</div>
                </div>
              ))
            )}
            <div ref={messagesEndRef} />
          </div>
          <div className="composer">
            <textarea
              id="message_input"
              value={messageInput}
              onChange={(e) => setMessageInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey && !isSending) {
                  e.preventDefault();
                  handleSendMessage();
                }
              }}
              placeholder="Escribe el mensaje para el agente seleccionado... (Ctrl+Enter para enviar)"
              disabled={!selectedAgent || !isConnected}
            />
            <div className="composer-actions">
              <button
                className="button primary"
                onClick={handleSendMessage}
                disabled={!selectedAgent || !isConnected || !messageInput.trim() || isSending}
                type="button"
              >
                {isSending ? 'Enviando...' : 'Enviar mensaje'}
              </button>
              <button className="button" onClick={() => setMessageInput('')} type="button">
                Limpiar
              </button>
              <label className="badge">
                <input
                  type="checkbox"
                  checked={forceNewConversation}
                  onChange={(e) => setForceNewConversation(e.target.checked)}
                  style={{ accentColor: 'var(--accent)' }}
                />
                Forzar conversación nueva
              </label>
            </div>
            <div className="status-line" id="status-line">
              {status}
            </div>
          </div>
        </div>
      </main>

      <section className="panel inspector">
        <div className="panel-header">
          <div>
            <h3>Inspector</h3>
            <p>Configuración, persistencia y actividad reciente</p>
          </div>
        </div>
        <div className="inspector-body">
          <div className="stat-grid">
            <div className="stat">
              <label>Agentes</label>
              <strong>{agents.length}</strong>
            </div>
            <div className="stat">
              <label>Mensajes</label>
              <strong>{messages.length}</strong>
            </div>
            <div className="stat">
              <label>Estado</label>
              <strong>{status}</strong>
            </div>
            <div className="stat">
              <label>Conversación</label>
              <strong className="mono">{conversationId || '-'}</strong>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
