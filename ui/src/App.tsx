import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import './styles/main.css';
// Componentes funcionales
import FullChatInterface from './components/Chat/FullChatInterface';
import ProjectList from './components/Project/ProjectList';
import NarrativeGenerator from './components/Generation/NarrativeGenerator';
import Dashboard from './components/Common/Dashboard';

function App() {
  return (
    <Router>
      <div className="app">
        <header className="header">
          <h1>Cadiz12 SDK</h1>
          <nav className="nav">
            <Link to="/">Home</Link>
            <Link to="/projects">Projects</Link>
            <Link to="/agents">Agents</Link>
            <Link to="/narratives">Narratives</Link>
          </nav>
        </header>
        
        <main className="main">
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/projects" element={<ProjectsPage />} />
            <Route path="/agents" element={<AgentsPage />} />
            <Route path="/narratives" element={<NarrativesPage />} />
          </Routes>
        </main>
        
        <footer className="footer">
          <p>Cadiz12 SDK v0.1.0 - Unified Agent Console for Cadiz12</p>
        </footer>
      </div>
    </Router>
  );
}

function HomePage() {
  return (
    <div className="page">
      <Dashboard />
    </div>
  );
}

function ProjectsPage() {
  return (
    <div className="page">
      <ProjectList />
    </div>
  );
}

function AgentsPage() {
  return (
    <div className="page" style={{ height: 'calc(100vh - 120px)' }}>
      <FullChatInterface />
    </div>
  );
}

function NarrativesPage() {
  return (
    <div className="page">
      <NarrativeGenerator />
    </div>
  );
}

export default App;
