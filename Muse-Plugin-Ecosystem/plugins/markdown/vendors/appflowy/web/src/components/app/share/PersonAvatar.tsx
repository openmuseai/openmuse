import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';

interface PersonAvatarProps {
  avatarUrl?: string;
  name: string;
  size?: number;
}

export function PersonAvatar({ avatarUrl, name, size }: PersonAvatarProps) {
  return (
    <Avatar style={size ? { width: size, height: size, fontSize: size } : undefined}>
      <AvatarImage src={avatarUrl} />
      <AvatarFallback
        name={name}
        style={{
          fontSize: size ? size * 0.75 : undefined,
        }}
      >
        {avatarUrl ? <span className='text-lg'>{avatarUrl}</span> : name}
      </AvatarFallback>
    </Avatar>
  );
}