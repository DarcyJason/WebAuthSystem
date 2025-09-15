import { createFileRoute, Navigate } from "@tanstack/react-router";
import { useAuth } from "../contexts/AuthContext";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "../components/ui/card";

export const Route = createFileRoute("/me")({
  component: MePage,
});

function MePage() {
  const { user, isLoading } = useAuth();

  // 在 AuthContext 完成初始状态检查前，显示加载中
  if (isLoading) {
    return <div>Loading...</div>;
  }

  // 如果没有用户信息（未登录），则重定向到登录页
  if (!user) {
    return <Navigate to="/login" />;
  }

  return (
    <div className="p-4 flex justify-center">
      <div className="w-full max-w-md">
        <Card>
          <CardHeader>
            <CardTitle>My Profile</CardTitle>
            <CardDescription>
              Your user information is displayed below.
            </CardDescription>
          </CardHeader>
          <CardContent className="grid gap-4">
            <div className="flex items-center justify-between border-b pb-2">
              <span className="text-sm text-muted-foreground">ID</span>
              <span className="text-sm font-mono">{user.id}</span>
            </div>
            <div className="flex items-center justify-between border-b pb-2">
              <span className="text-sm text-muted-foreground">Name</span>
              <span className="text-sm font-medium">{user.name}</span>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
