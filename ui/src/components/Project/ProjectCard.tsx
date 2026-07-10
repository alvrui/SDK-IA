import { Project } from '../../types';
import { Button } from '../Common';

interface ProjectCardProps {
  project: Project;
  onEdit: () => void;
  onDelete: () => void;
}

export default function ProjectCard({ project, onEdit, onDelete }: ProjectCardProps) {
  const statusColors = {
    draft: 'bg-yellow-100 text-yellow-800',
    active: 'bg-green-100 text-green-800',
    archived: 'bg-gray-100 text-gray-800',
  };

  return (
    <div className="bg-white p-4 rounded-lg shadow-md border">
      <div className="flex justify-between items-start">
        <div>
          <h3 className="font-semibold text-lg">{project.name}</h3>
          <span className={`inline-block px-2 py-1 text-xs rounded-full ${statusColors[project.status]}`}>
            {project.status}
          </span>
        </div>
      </div>
      <p className="text-gray-600 text-sm mt-2">{project.description || 'No description'}</p>
      <div className="flex justify-between items-center mt-4">
        <span className="text-xs text-gray-500">{new Date(project.created_at).toLocaleDateString()}</span>
        <div className="space-x-2">
          <Button size="sm" variant="secondary" onClick={onEdit}>
            Edit
          </Button>
          <Button size="sm" variant="danger" onClick={onDelete}>
            Delete
          </Button>
        </div>
      </div>
    </div>
  );
}