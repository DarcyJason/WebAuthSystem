import { createFileRoute, Link } from "@tanstack/react-router";
import { useAuth } from "../contexts/AuthContext";
import { Button } from "../components/ui/button";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  const { isAuthenticated, user } = useAuth();

  return (
    <div className="container mx-auto text-center py-20">
      <h1 className="text-4xl md:text-5xl font-bold tracking-tight mb-4">
        Welcome to Web Auth System
      </h1>
      <p className="text-lg md:text-xl text-muted-foreground mb-10">
        A modern and secure web authentication system built with React and Rust.
      </p>

      {isAuthenticated ? (
        <div>
          <p className="text-lg mb-4">
            You are logged in as <strong>{user?.name}</strong>.
          </p>
          <Link to="/me">
            <Button size="lg">Go to My Profile</Button>
          </Link>
        </div>
      ) : (
        <div className="flex justify-center items-center gap-4">
          <Link to="/login">
            <Button size="lg">Login</Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline">
              Register
            </Button>
          </Link>
        </div>
      )}
    </div>
  );
}
