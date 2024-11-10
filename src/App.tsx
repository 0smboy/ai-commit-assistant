// src/App.tsx
import { invoke } from "@tauri-apps/api/tauri";
import { Settings, GitCommit, RefreshCcw, Copy, FolderOpen } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "./components/ui/alert";
import { useState, useEffect } from "react";

const App = () => {
  const [apiKey, setApiKey] = useState("");
  const [provider, setProvider] = useState("openai");
  const [commitMessage, setCommitMessage] = useState("");
  const [loading, setLoading] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [notification, setNotification] = useState(null);

  const handleGenerateCommit = async () => {
    setLoading(true);
    try {
      // 模拟API调用
      await new Promise(resolve => setTimeout(resolve, 1000));
      setCommitMessage("feat: update react-router-dom and related dependencies");
      setNotification({
        type: "success",
        message: "Commit message generated successfully!"
      });
    } catch (error) {
      setNotification({
        type: "error",
        message: "Failed to generate commit message"
      });
    }
    setLoading(false);
  };

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setNotification({
        type: "success",
        message: "Copied to clipboard!"
      });
    } catch (error) {
      setNotification({
        type: "error",
        message: "Failed to copy to clipboard"
      });
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="bg-gray-800 p-4">
        <div className="container mx-auto flex justify-between items-center">
          <h1 className="text-xl font-bold">AI Commit Assistant</h1>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="p-2 hover:bg-gray-700 rounded-full transition-colors"
          >
            <Settings className="w-6 h-6" />
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto p-6">
        {showSettings && (
          <div className="mb-8 bg-gray-800 p-6 rounded-lg">
            <h2 className="text-lg font-semibold mb-4">Settings</h2>
            <div className="space-y-4">
              <div>
                <label className="block mb-2">API Provider</label>
                <select
                  value={provider}
                  onChange={(e) => setProvider(e.target.value)}
                  className="w-full bg-gray-700 p-2 rounded"
                >
                  <option value="openai">OpenAI</option>
                  <option value="anthropic">Anthropic</option>
                </select>
              </div>
              <div>
                <label className="block mb-2">API Key</label>
                <input
                  type="password"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  className="w-full bg-gray-700 p-2 rounded"
                  placeholder="Enter your API key"
                />
              </div>
            </div>
          </div>
        )}

        <div className="bg-gray-800 p-6 rounded-lg">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-lg font-semibold">Generate Commit</h2>
            <button
              onClick={handleGenerateCommit}
              disabled={loading}
              className="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded transition-colors disabled:opacity-50"
            >
              {loading ? (
                <RefreshCcw className="w-5 h-5 animate-spin" />
              ) : (
                <GitCommit className="w-5 h-5" />
              )}
              Generate
            </button>
          </div>

          {commitMessage && (
            <div className="space-y-4">
              <div className="bg-gray-900 p-4 rounded">
                <p className="font-mono">{commitMessage}</p>
              </div>
              <button
                onClick={() => copyToClipboard(commitMessage)}
                className="flex items-center gap-2 bg-gray-700 hover:bg-gray-600 px-4 py-2 rounded transition-colors"
              >
                <Copy className="w-4 h-4" />
                Copy to Clipboard
              </button>
            </div>
          )}
        </div>

        {notification && (
          <div className="fixed bottom-6 right-6">
            <Alert
              className={`${
                notification.type === "error"
                  ? "border-red-500 bg-red-500/10"
                  : "border-green-500 bg-green-500/10"
              }`}
            >
              {notification.type === "error" && (
                <AlertTitle>Error</AlertTitle>
              )}
              <AlertDescription>{notification.message}</AlertDescription>
            </Alert>
          </div>
        )}
      </main>
    </div>
  );
};

export default App;
