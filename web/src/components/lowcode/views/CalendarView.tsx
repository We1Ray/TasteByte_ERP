"use client";

import { useMemo, useCallback } from "react";
import FullCalendar from "@fullcalendar/react";
import dayGridPlugin from "@fullcalendar/daygrid";
import timeGridPlugin from "@fullcalendar/timegrid";
import interactionPlugin from "@fullcalendar/interaction";
import type { EventClickArg } from "@fullcalendar/core";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { executorApi } from "@/lib/api/lowcode";
import { Card } from "@/components/ui/card";
import { PageLoading } from "@/components/ui/loading";
import type { CalendarViewConfig, FormRecord } from "@/lib/types/lowcode";
import type { PaginatedResponse } from "@/lib/api/client";

interface CalendarViewProps {
  operationCode: string;
  config: CalendarViewConfig;
  onEventClick?: (record: FormRecord) => void;
  onDateClick?: (dateStr: string) => void;
}

export function CalendarView({
  operationCode,
  config,
  onEventClick,
  onDateClick,
}: CalendarViewProps) {
  const { data, isLoading } = useApiQuery<PaginatedResponse<FormRecord>>(
    ["lowcode", "data", operationCode, "calendar"],
    () => executorApi.list(operationCode, { page_size: 1000 })
  );

  const events = useMemo(() => {
    const records = data?.items ?? [];
    return records
      .filter((r) => r.data[config.dateField])
      .map((record) => ({
        id: record.id,
        title: String(record.data[config.titleField] || "Untitled"),
        start: String(record.data[config.dateField]),
        backgroundColor: config.colorField
          ? String(record.data[config.colorField] || "#3B82F6")
          : "#3B82F6",
        borderColor: "transparent",
        extendedProps: { record },
      }));
  }, [data, config.dateField, config.titleField, config.colorField]);

  const handleEventClick = useCallback(
    (info: EventClickArg) => {
      const record = info.event.extendedProps.record as FormRecord;
      onEventClick?.(record);
    },
    [onEventClick]
  );

  const handleDateClick = useCallback(
    (info: { dateStr: string }) => {
      onDateClick?.(info.dateStr);
    },
    [onDateClick]
  );

  if (isLoading) {
    return <PageLoading />;
  }

  return (
    <Card className="overflow-hidden">
      <div className="calendar-wrapper">
        <FullCalendar
          plugins={[dayGridPlugin, timeGridPlugin, interactionPlugin]}
          initialView="dayGridMonth"
          headerToolbar={{
            left: "prev,next today",
            center: "title",
            right: "dayGridMonth,timeGridWeek,timeGridDay",
          }}
          events={events}
          eventClick={handleEventClick}
          dateClick={handleDateClick}
          height="auto"
          dayMaxEvents={3}
          eventDisplay="block"
          eventTimeFormat={{
            hour: "2-digit",
            minute: "2-digit",
            meridiem: false,
          }}
        />
      </div>

      <style jsx global>{`
        .calendar-wrapper .fc {
          font-family: inherit;
        }
        .calendar-wrapper .fc .fc-toolbar-title {
          font-size: 1.125rem;
          font-weight: 600;
        }
        .calendar-wrapper .fc .fc-button {
          padding: 0.25rem 0.75rem;
          font-size: 0.875rem;
          font-weight: 500;
          border-radius: 0.375rem;
          background-color: #fff;
          border: 1px solid #d1d5db;
          color: #374151;
          box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
        }
        .calendar-wrapper .fc .fc-button:hover {
          background-color: #f9fafb;
        }
        .calendar-wrapper .fc .fc-button-active,
        .calendar-wrapper .fc .fc-button:active {
          background-color: #2563eb !important;
          border-color: #2563eb !important;
          color: #fff !important;
        }
        .calendar-wrapper .fc .fc-button:focus {
          box-shadow: 0 0 0 2px #fff, 0 0 0 4px #3b82f6;
        }
        .calendar-wrapper .fc .fc-today-button:disabled {
          opacity: 0.5;
        }
        .calendar-wrapper .fc .fc-daygrid-day-top {
          padding: 0.25rem 0.5rem;
        }
        .calendar-wrapper .fc .fc-daygrid-day.fc-day-today {
          background-color: #eff6ff;
        }
        .calendar-wrapper .fc .fc-event {
          border-radius: 0.25rem;
          padding: 1px 4px;
          font-size: 0.75rem;
          cursor: pointer;
        }
        .calendar-wrapper .fc .fc-col-header-cell-cushion {
          font-size: 0.75rem;
          font-weight: 600;
          text-transform: uppercase;
          color: #6b7280;
          padding: 0.5rem 0;
        }
        .calendar-wrapper .fc th {
          border-color: #e5e7eb;
        }
        .calendar-wrapper .fc td {
          border-color: #e5e7eb;
        }
      `}</style>
    </Card>
  );
}
