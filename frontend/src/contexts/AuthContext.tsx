import {
  createContext,
  useContext,
  useState,
  useEffect,
  type ReactNode,
} from "react";
import api from "../lib/api";

interface User {
  id: string;
  name: string;
  email: string;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  login: () => Promise<void>; // login 函数不再接收参数，但返回一个 Promise
  logout: () => Promise<void>;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const mapRawUserToUser = (rawUserData: any): User | null => {
  if (!rawUserData || !rawUserData.id?.id?.String) {
    return null;
  }
  return {
    id: rawUserData.id.id.String,
    name: rawUserData.name,
    email: rawUserData.email,
  };
};

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const fetchUser = async () => {
    try {
      const response = await api.get("/user/me");
      const mappedUser = mapRawUserToUser(response.data.data);
      setUser(mappedUser);
    } catch (error) {
      setUser(null);
    }
  };

  useEffect(() => {
    const initialCheck = async () => {
      await fetchUser();
      setIsLoading(false);
    };
    initialCheck();
  }, []);

  // login 函数现在负责触发用户信息的获取
  const login = async () => {
    await fetchUser();
  };

  const logout = async () => {
    try {
      await api.post("/auth/logout");
    } catch (error) {
      console.error("Server logout failed", error);
    } finally {
      setUser(null);
    }
  };

  return (
    <AuthContext.Provider
      value={{ user, isAuthenticated: !!user, login, logout, isLoading }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
