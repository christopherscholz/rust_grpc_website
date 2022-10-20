import React from 'react';
import ReactDOM from 'react-dom/client';
import './css/normalize.css';
import './css/index.css';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Page from "./pages/Page";
import NoPage from "./pages/NoPage";

const root = ReactDOM.createRoot(document.getElementById('root'));

root.render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route index element={<Page page="home" />} />
                <Route path="impressum" element={<Page page="impressum" />} />
                <Route path="*" element={<NoPage />} />
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
);
