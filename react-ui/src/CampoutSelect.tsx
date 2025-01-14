import {useMemo} from "react";
import {useEventList} from "./api";
import {Event} from "./Event";
import {Form, Spinner} from "react-bootstrap";
import {CampoutOption} from "./CampoutOption";

interface Props {
    start: Date;
    end?: Date;
    onSelect?: (event: Event, eventId: number) => void;
}

export function CampoutSelect({start, end, onSelect}: Props) {
    const query = useEventList(start, end);
    const eventCmp = (a: Event, b: Event) => {
        const x = a?.activity_at ? parseInt(a.activity_at) : 0;
        const y = b?.activity_at ? parseInt(b.activity_at) : 0;
        // console.log(`x: ${x}, y: ${y}: ${x-y}`);
        return x - y;
    };
    const sorted = useMemo<Event[] | undefined>(() => {
       return query?.data?.events?.sort(eventCmp);
    }, [query?.data]);
    return (
        <>
            {query.isLoading ?
                <Spinner animation={'border'}/>
                :
                <Form>
                    <Form.Select onChange={e => {
                        const eventId = parseInt(e.currentTarget.value);
                        console.log(`selected eventId ${eventId}`);
                        const event = sorted?.find(e => e.event_id === eventId);
                        if (event) {
                            onSelect?.(event, eventId);
                            console.log(`selected event: ${JSON.stringify(event)}`);
                        }
                    }}>
                        <option key={'dummy'}>Choose a Campout</option>
                        {query.data?.events
                            ?.filter((e: Event) => e.event_type === "Campout")
                            ?.map((e: Event) => (
                                <CampoutOption key={e.event_id} event={e}/>
                            ))
                        }
                    </Form.Select>
                </Form>
            }
            {query.isError
                ?
                <div>Error: {query.error?.message} ({query.error?.code})</div>
                : null}
        </>
    );
}