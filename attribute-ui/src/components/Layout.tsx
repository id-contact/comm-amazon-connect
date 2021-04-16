import React from 'react';

export default function Layout({ children }) {
  return (
    <>
      <h1>ID Contact</h1>
      <div className="content">
        {children}
      </div>
    </>
  );
}
