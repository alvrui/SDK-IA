import React from 'react';
import { Link } from 'react-router-dom';

export default function Dashboard() {
  return (
    <div className="dashboard">
      <h2>Welcome to Cadiz12 SDK Dashboard</h2>
      <p>Unified application for narrative and event generation.</p>
      
      <div className="dashboard-cards">
        <div className="dashboard-card">
          <h3>Quick Actions</h3>
          <div className="dashboard-actions">
            <Link to="/projects" className="btn btn-primary">
              Manage Projects
            </Link>
            <Link to="/agents" className="btn btn-primary">
              Manage Agents
            </Link>
            <Link to="/narratives" className="btn btn-primary">
              Generate Narratives
            </Link>
          </div>
        </div>
        
        <div className="dashboard-card">
          <h3>System Status</h3>
          <div className="status-indicators">
            <div className="status-item">
              <span className="status-label">Rust Backend:</span>
              <span className="status-value" id="rust-status">Checking...</span>
            </div>
            <div className="status-item">
              <span className="status-label">Python Service:</span>
              <span className="status-value" id="python-status">Checking...</span>
            </div>
            <div className="status-item">
              <span className="status-label">UI Frontend:</span>
              <span className="status-value status-healthy">Running</span>
            </div>
          </div>
        </div>
        
        <div className="dashboard-card">
          <h3>Recent Activity</h3>
          <p>No recent activity yet.</p>
        </div>
      </div>
    </div>
  );
}
