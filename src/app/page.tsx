"use client";

import { invoke }  from "@tauri-apps/api/core";
import { useState } from "react";

export default function Home() {
  const [mensaje, setMensaje] = useState("");

  async function manejarClick() {
    try {
      const respuesta = await invoke<string>("saludar", { nombre: "Joan" });
      setMensaje(respuesta);
    } catch (err) {
      console.error("Error al invocar comando:", err);
    }
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gray-900 text-white">
      <h1 className="text-3xl font-bold mb-6">
        🚀 Mi App de Escritorio con Next.js + Tauri
      </h1>
      <button
        onClick={manejarClick}
        className="bg-blue-600 hover:bg-blue-800 px-4 py-2 rounded-lg shadow-lg"
      >
        Decir Hola desde Rust
      </button>
      {mensaje && <p className="mt-4 text-lg">{mensaje}</p>}
    </main>
  );
}
