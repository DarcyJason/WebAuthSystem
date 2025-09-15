import { Link, useNavigate } from "@tanstack/react-router";
import { useAuth } from "../contexts/AuthContext";
import { useNotification } from "../contexts/NotificationContext";
import { Button } from "./ui/button";

export default function Header() {
  const { isAuthenticated, user, logout, isLoading } = useAuth();
  const { addNotification } = useNotification();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    addNotification("You have been successfully logged out.", "info");
    navigate({ to: "/" });
  };

  return (
    <header className="p-4 bg-gray-100 border-b">
      <div className="container mx-auto flex justify-between items-center">
        <Link to="/" className="font-bold text-xl">
          Web Auth System
        </Link>
        <nav>
          {isLoading ? (
            <div>Loading...</div>
          ) : isAuthenticated ? (
            <div className="flex items-center gap-4">
              <Link to="/me" className="hover:underline">
                <span>Welcome, {user?.name}</span>
              </Link>
              <Button onClick={handleLogout} variant="outline">
                Logout
              </Button>
            </div>
          ) : (
            <div className="flex items-center gap-4">
              <Link to="/login">
                <Button variant="outline">Login</Button>
              </Link>
              <Link to="/register">
                <Button>Sign Up</Button>
              </Link>
            </div>
          )}
        </nav>
      </div>
    </header>
  );
}
