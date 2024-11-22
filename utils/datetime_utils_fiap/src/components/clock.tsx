import React from "react";
import "./clock.css";

interface ClockProps {
  customText: string;
  dateTime: string;
}

const Clock: React.FC<ClockProps> = (ClockProps) => {


  if (!ClockProps.dateTime) {
    return null; // Não renderiza nada enquanto dateTime não estiver disponível
  }
  return (
    <div className="clock" data-tauri-drag-region >
      <div className="datetime" data-tauri-drag-region>
        {ClockProps.dateTime.split(" ").map((part, index) => {
          return <span data-tauri-drag-region className="text_gray_neon digital_font" key={index}>{part}</span>;
        })}
      </div>
      <div className="user_hostname">
        <textarea className="" name="" id=""></textarea>
      </div>
    </div>
  );
};

export default Clock;