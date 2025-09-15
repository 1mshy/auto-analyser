import React, { useState } from "react";
import { Button } from "@heroui/button";
import { Navbar, NavbarBrand, NavbarContent, NavbarItem } from "@heroui/navbar";

import { useAuth } from "@/contexts/AuthContext";
import { StockQuote } from "@/components/StockQuote";
import { Watchlist } from "@/components/Watchlist";
import { AlertsManager } from "@/components/AlertsManager";

type TabType = "quotes" | "watchlist" | "alerts";

export const Dashboard: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>("quotes");
  const { user, logout } = useAuth();

  const renderContent = () => {
    switch (activeTab) {
      case "quotes":
        return <StockQuote />;
      case "watchlist":
        return <Watchlist />;
      case "alerts":
        return <AlertsManager />;
      default:
        return <StockQuote />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <Navbar maxWidth="full" className="border-b">
        <NavbarBrand>
          <h1 className="text-xl font-bold text-primary">Equity Analyser</h1>
        </NavbarBrand>
        
        <NavbarContent className="hidden sm:flex gap-4" justify="center">
          <NavbarItem>
            <Button
              variant={activeTab === "quotes" ? "solid" : "light"}
              color={activeTab === "quotes" ? "primary" : "default"}
              onPress={() => setActiveTab("quotes")}
            >
              Market Data
            </Button>
          </NavbarItem>
          <NavbarItem>
            <Button
              variant={activeTab === "watchlist" ? "solid" : "light"}
              color={activeTab === "watchlist" ? "primary" : "default"}
              onPress={() => setActiveTab("watchlist")}
            >
              Watchlist
            </Button>
          </NavbarItem>
          <NavbarItem>
            <Button
              variant={activeTab === "alerts" ? "solid" : "light"}
              color={activeTab === "alerts" ? "primary" : "default"}
              onPress={() => setActiveTab("alerts")}
            >
              Alerts
            </Button>
          </NavbarItem>
        </NavbarContent>

        <NavbarContent justify="end">
          <NavbarItem className="flex items-center gap-4">
            <span className="text-sm text-gray-600">
              Welcome, {user?.username}
            </span>
            <Button
              color="danger"
              variant="light"
              size="sm"
              onPress={logout}
            >
              Logout
            </Button>
          </NavbarItem>
        </NavbarContent>
      </Navbar>

      <main className="container mx-auto px-4 py-8">
        <div className="max-w-6xl mx-auto">
          {renderContent()}
        </div>
      </main>

      {/* Mobile Tab Navigation */}
      <div className="sm:hidden fixed bottom-0 left-0 right-0 bg-white border-t">
        <div className="flex">
          <Button
            fullWidth
            variant={activeTab === "quotes" ? "solid" : "light"}
            color={activeTab === "quotes" ? "primary" : "default"}
            className="rounded-none"
            onPress={() => setActiveTab("quotes")}
          >
            Quotes
          </Button>
          <Button
            fullWidth
            variant={activeTab === "watchlist" ? "solid" : "light"}
            color={activeTab === "watchlist" ? "primary" : "default"}
            className="rounded-none"
            onPress={() => setActiveTab("watchlist")}
          >
            Watchlist
          </Button>
          <Button
            fullWidth
            variant={activeTab === "alerts" ? "solid" : "light"}
            color={activeTab === "alerts" ? "primary" : "default"}
            className="rounded-none"
            onPress={() => setActiveTab("alerts")}
          >
            Alerts
          </Button>
        </div>
      </div>
    </div>
  );
};
