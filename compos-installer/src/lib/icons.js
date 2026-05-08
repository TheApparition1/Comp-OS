import {
  AppWindow,
  BadgeCheck,
  Briefcase,
  Code2,
  FileText,
  Gamepad2,
  Mail,
  Monitor,
  Music2,
  Shield,
  TriangleAlert,
  Wrench
} from 'lucide-svelte';

export const categoryIcons = {
  music: Music2,
  office: Briefcase,
  email: Mail,
  editor: FileText,
  ide: Code2,
  graphics: AppWindow,
  development: Wrench,
  security: Shield,
  communication: Mail,
  media: Music2,
  productivity: Briefcase,
  virtualization: Monitor,
  gaming: Gamepad2
};

export const statusIcons = {
  selected: BadgeCheck,
  warning: TriangleAlert
};
