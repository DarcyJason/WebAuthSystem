import { useEffect } from 'react';
import { useNotification, type NotificationType } from '../contexts/NotificationContext';
import { Alert, AlertDescription, AlertTitle } from './ui/alert';

const notificationStyles: Record<NotificationType, string> = {
  success: 'bg-green-50 border-green-200 text-green-800',
  info: 'bg-blue-50 border-blue-200 text-blue-800',
  error: 'bg-red-50 border-red-200 text-red-800',
};

function NotificationItem({ id, message, type }: { id: number; message: string; type: NotificationType }) {
  const { removeNotification } = useNotification();

  useEffect(() => {
    const timer = setTimeout(() => {
      removeNotification(id);
    }, 5000); // 5秒后自动消失

    return () => {
      clearTimeout(timer);
    };
  }, [id, removeNotification]);

  return (
    <Alert className={`${notificationStyles[type]} relative pr-10`}>
      <AlertTitle className="capitalize">{type}</AlertTitle>
      <AlertDescription>{message}</AlertDescription>
      <button
        onClick={() => removeNotification(id)}
        className="absolute top-2 right-2 text-lg font-bold opacity-50 hover:opacity-100"
      >
        &times;
      </button>
    </Alert>
  );
}

export default function Notifications() {
  const { notifications } = useNotification();

  return (
    <div className="fixed bottom-4 right-4 w-80 z-50 flex flex-col gap-3">
      {notifications.map((n) => (
        <NotificationItem key={n.id} {...n} />
      ))}
    </div>
  );
}
