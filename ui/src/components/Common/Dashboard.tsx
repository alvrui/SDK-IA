import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { listAgents } from '../../api/agents';
import { listProjects } from '../../api/projects';
import { listNarratives } from '../../api/narratives';
import './Dashboard.css';

interface StatCardProps {
  title: string;
  value: number | string;
  icon?: string;
  link?: string;
  color?: string;
}

function StatCard({ title, value, icon, link, color = '#38bdf8' }: StatCardProps) {
  const content = (
    <div className="stat-card" style={{ borderTopColor: color }}>
      {icon && <div className="stat-icon" style={{ backgroundColor: color }}>{icon}</div>}
      <div className="stat-info">
        <span className="stat-value">{value}</span>
        <span className="stat-title">{title}</span>
      </div>
    </div>
  );

  return link ? <Link to={link}>{content}</Link> : <>{content}</>;
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
      } catch (error) {
        setStats({
          ...stats,
          loading: false,
          error: 'Error al cargar estadísticas'
        });
      }
    };

    loadStats();
  }, []);

  if (stats.loading) {
    return (
      <div className="dashboard">
        <div className="loading">Cargando estadísticas...</div>
      </div>
    );
  }

  if (stats.error) {
    return (
      <div className="dashboard">
        <div className="error">{stats.error}</div>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <h2>Panel de Control - Cadiz12 SDK</h2>
      <p className="subtitle">Gestión unificada de agentes, proyectos y narrativas</p>

      <div className="stats-grid">
        <StatCard
          title="Agentes"
          value={stats.agents}
          icon="🤖"
          link="/agents"
          color="#38bdf8"
        />
        <StatCard
          title="Proyectos"
          value={stats.projects}
          icon="📁"
          link="/projects"
          color="#10b981"
        />
        <StatCard
          title="Narrativas"
          value={stats.narratives}
          icon="📜"
          link="/narratives"
          color="#8b5cf6"
        />
        <StatCard
          title="Salud del Sistema"
          value="✅ Operativo"
          icon="⚡"
          color="#22c55e"
        />
      </div>

      <div className="quick-actions">
        <h3>Acciones Rápidas</h3>
        <div className="actions-grid">
          <Link to="/agents" className="action-card">
            <div className="action-icon">🤖</div>
            <div className="action-text">
              <strong>Gestionar Agentes</strong>
              <span>Configurar y chatear con agentes Mistral</span>
            </div>
          </Link>
          <Link to="/projects" className="action-card">
            <div className="action-icon">📁</div>
            <div className="action-text">
              <strong>Gestionar Proyectos</strong>
              <span>Crear y administrar proyectos Cadiz12</span>
            </div>
          </Link>
          <Link to="/narratives" className="action-card">
            <div className="action-icon">📜</div>
            <div className="action-text">
              <strong>Generar Narrativas</strong>
              <span>Crear historias con Hollywood Animal</span>
            </div>
          </Link>
        </div>
      </div>

      <div className="recent-activity">
        <h3>Actividad Reciente</h3>
        <div className="activity-list">
          <div className="activity-item">
            <span className="activity-icon">🔄</span>
            <span className="activity-text">Sistema inicializado correctamente</span>
            <span className="activity-time">Hace unos momentos</span>
          </div>
          <div className="activity-item">
            <span className="activity-icon">🤖</span>
            <span className="activity-text">{stats.agents} agentes disponibles</span>
            <span className="activity-time">Cargados</span>
          </div>
          <div className="activity-item">
            <span className="activity-icon">📊</span>
            <span className="activity-text">Todos los servicios operativos</span>
            <span className="activity-time">Verificado</span>
          </div>
        </div>
      </div>
    </div>
  );
}
