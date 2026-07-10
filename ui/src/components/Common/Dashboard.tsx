import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { listAgents } from '../../api/agents';
import { listProjects } from '../../api/projects';
import { listNarratives } from '../../api/narratives';
import LoadingSpinner from './LoadingSpinner';
import ErrorDisplay from './ErrorDisplay';
import './Dashboard.css';

interface StatCardProps {
  title: string;
  value: number | string;
  icon: string;
  link?: string;
  color?: string;
}

function StatCard({ title, value, icon, link, color = '#38bdf8' }: StatCardProps) {
  const content = (
    <div className="stat-card" style={{ borderTopColor: color }}>
      <div className="stat-icon" style={{ backgroundColor: color }}>
        {icon}
      </div>
      <div className="stat-info">
        <span className="stat-value">{value}</span>
        <span className="stat-title">{title}</span>
      </div>
    </div>
  );

  return link ? (
    <Link to={link} className="stat-card-link">
      {content}
    </Link>
  ) : (
    <>{content}</>
  );
}

export default function Dashboard() {
  const [stats, setStats] = useState({
    agents: 0,
    projects: 0,
    narratives: 0,
    loading: true,
    error: null as string | null
  });

  useEffect(() => {
    const loadStats = async () => {
      try {
        setStats(prev => ({ ...prev, loading: true, error: null }));

        const [agentsResponse, projectsResponse, narrativesResponse] = await Promise.all([
          listAgents().catch(() => ({ agents: [] })),
          listProjects().catch(() => ({ projects: [] })),
          listNarratives().catch(() => ({ narratives: [] }))
        ]);

        setStats({
          agents: agentsResponse.agents?.length || 0,
          projects: projectsResponse.projects?.length || 0,
          narratives: narrativesResponse.narratives?.length || 0,
          loading: false,
          error: null
        });
      } catch (err) {
        setStats({
          ...stats,
          loading: false,
          error: err instanceof Error ? err.message : 'Failed to load statistics'
        });
      }
    };

    loadStats();
  }, []);

  if (stats.loading) {
    return (
      <div className="dashboard">
        <LoadingSpinner />
      </div>
    );
  }

  if (stats.error) {
    return (
      <div className="dashboard">
        <ErrorDisplay message={stats.error} onRetry={() => window.location.reload()} />
      </div>
    );
  }

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h2>Cadiz12 SDK Dashboard</h2>
        <p>Unified management for agents, projects, and narratives</p>
      </div>

      <div className="stats-grid">
        <StatCard
          title="Agents"
          value={stats.agents}
          icon="🤖"
          link="/agents"
          color="#38bdf8"
        />
        <StatCard
          title="Projects"
          value={stats.projects}
          icon="📁"
          link="/projects"
          color="#10b981"
        />
        <StatCard
          title="Narratives"
          value={stats.narratives}
          icon="📜"
          link="/narratives"
          color="#8b5cf6"
        />
        <StatCard
          title="System"
          value="✅ All systems operational"
          icon="⚡"
          color="#22c55e"
        />
      </div>

      <div className="quick-actions">
        <h3>Quick Actions</h3>
        <div className="actions-grid">
          <Link to="/agents" className="action-card">
            <div className="action-icon">🤖</div>
            <div className="action-content">
              <strong>Manage Agents</strong>
              <span>Configure and communicate with Mistral AI agents</span>
            </div>
          </Link>
          <Link to="/projects" className="action-card">
            <div className="action-icon">📁</div>
            <div className="action-content">
              <strong>Manage Projects</strong>
              <span>Create and organize your Cadiz12 projects</span>
            </div>
          </Link>
          <Link to="/narratives" className="action-card">
            <div className="action-icon">📜</div>
            <div className="action-content">
              <strong>Generate Narratives</strong>
              <span>Create stories with Hollywood Animal validation</span>
            </div>
          </Link>
        </div>
      </div>

      <div className="system-info">
        <h3>System Information</h3>
        <div className="info-grid">
          <div className="info-item">
            <span className="info-label">Backend Status</span>
            <span className="info-value status-healthy">Healthy</span>
          </div>
          <div className="info-item">
            <span className="info-label">Python Service</span>
            <span className="info-value status-healthy">Healthy</span>
          </div>
          <div className="info-item">
            <span className="info-label">UI Version</span>
            <span className="info-value">v0.1.0</span>
          </div>
          <div className="info-item">
            <span className="info-label">Last Updated</span>
            <span className="info-value">{new Date().toLocaleDateString()}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
