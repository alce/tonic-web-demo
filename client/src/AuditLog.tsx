import { ClientReadableStream } from "grpc-web";
import React, { FC, useCallback, useEffect, useState } from "react";
import { Action, Event, SubscribeRequest } from "./pb/audit_log_pb";
import { AuditLogServiceClient } from "./pb/Audit_logServiceClientPb";
import { Product } from "./pb/products_pb";

const client = new AuditLogServiceClient("http://localhost:9999");

interface Props {
  onProductCreated: (product: Product) => void;
  onProductDeleted: (id: string) => void;
}

const eventDescription = (event: Event) => {
  const action = event.getAction() === Action.CREATED ? "created" : "deleted";

  const subject =
    event.getAction() === Action.CREATED
      ? event.getProduct()!.getName()
      : event.getProductId().substr(0, 4);

  return `${event.getUser()} ${action} ${subject} `;
};

export const AuditLog: FC<Props> = ({ onProductCreated, onProductDeleted }) => {
  const [events, setEvents] = useState<Event[]>([]);
  const [stream, setStream] = useState<ClientReadableStream<Event> | null>(
    null
  );

  const created = useCallback(onProductCreated, []);
  const deleted = useCallback(onProductDeleted, []);

  useEffect(() => {
    setStream(client.subscribe(new SubscribeRequest()));
  }, []);

  useEffect(() => {
    stream?.on("data", (event: Event) => {
      switch (event.getAction()) {
        case Action.CREATED:
          setEvents((prev) => [event, ...prev]);
          created(event.getProduct()!);
          break;
        case Action.DELETED:
          setEvents((prev) => [event, ...prev]);
          deleted(event.getProductId());
          break;
      }
    });
  }, [stream, created, deleted]);

  return (
    <div className="events">
      {events.length ? (
        events.map((event) => (
          <div key={event.getId()} className="event">
            <p className="mute">
              {event.getCreateTime()?.toDate().toLocaleTimeString()}
            </p>
            <p>{eventDescription(event)}</p>
          </div>
        ))
      ) : (
        <p>No recent activity</p>
      )}
    </div>
  );
};
