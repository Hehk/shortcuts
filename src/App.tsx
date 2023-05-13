import { For, JSX, createEffect, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { register, isRegistered } from "@tauri-apps/api/globalShortcut";
import "./App.css";

type Action =
  | {
      kind: "openApp";
      path: string;
    }
  | {
      kind: "script";
      command: string;
    };

type Keybind = {
  id: string;
  shortcut: string;
  action: Action;
};

function Action({ action }: { action: Action }): JSX.Element {
  switch (action.kind) {
    case "openApp":
      return <span>Open {action.path}</span>;
    case "script":
      return <span>Run {action.command}</span>;
  }
}

const defaultKeybinds: Keybind[] = [
  {
    id: "greet",
    shortcut: "Ctrl+1",
    action: {
      kind: "openApp",
      path: "/Applications/Arc.app",
    },
  },
  {
    id: "openApp",
    shortcut: "Ctrl+2",
    action: {
      kind: "openApp",
      path: "/Applications/Visual Studio Code.app",
    },
  },
  {
    id: "openApp",
    shortcut: "Ctrl+3",
    action: {
      kind: "openApp",
      path: "/Applications/Warp.app",
    },
  },
  {
    id: "openApp",
    shortcut: "Ctrl+4",
    action: {
      kind: "openApp",
      path: "/Applications/LogSeq.app",
    },
  },
  {
    id: "openApp",
    shortcut: "Ctrl+5",
    action: {
      kind: "openApp",
      path: "/Applications/Google Chrome.app",
    },
  },
];

function KeybindList() {
  const [keybinds, setKeybinds] = createSignal<Keybind[]>(defaultKeybinds);

  createEffect(() => {
    keybinds().forEach(async (keybind) => {
      if (await isRegistered(keybind.shortcut)) {
        return;
      }

      await register(keybind.shortcut, () => {
        switch (keybind.action.kind) {
          case "openApp":
            invoke("open_app", { path: keybind.action.path });
            break;
          case "script":
            invoke("runScript", { command: keybind.action.command });
            break;
        }
      });
    });
  });

  return (
    <For each={keybinds()}>
      {(keybind) => (
        <div>
          <span>{keybind.shortcut}</span>
          <Action action={keybind.action} />
        </div>
      )}
    </For>
  );
}

function App() {
  const [name, setName] = createSignal("");

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <KeybindList />
    </div>
  );
}

export default App;
