import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import './styles/main.css';

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
          <p>Cadiz12 SDK v0.1.0</p>
        </footer>
      </div>
    </Router>
  );
}

function HomePage() {
  return (
    <div className="page">
      <h2>Welcome to Cadiz12 SDK</h2>
      <p>Unified application for narrative and event generation.</p>
    </div>
  );
}

function ProjectsPage() {
  return (
    <div className="page">
      <h2>Projects</h2>
      <p>Manage your Cadiz12 projects here.</p>
    </div>
  );
}

function AgentsPage() {
  return (
    <div className="page">
      <h2>Agents</h2>
      <p>Manage Mistral AI agents here.</p>
    </div>
  );
}

function NarrativesPage() {
  return (
    <div className="page">
      <h2>Narratives</h2>
      <p>Generate and manage narratives here.</p>
    </div>
  );
}

export default App;
