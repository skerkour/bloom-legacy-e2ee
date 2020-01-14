import 'package:bloom/bloom/calendar/models/event.dart';

class CalendarListEvents {
  CalendarListEvents(this.startAt, this.endAt);
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'start_at': startAt.toUtc().toIso8601String(),
      'end_at': endAt.toUtc().toIso8601String(),
    };
    return data;
  }
}

class CalendarEvents {
  CalendarEvents({this.events});

  final List<Event> events;

  static CalendarEvents fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['events'];
    final List<Event> events =
        list.map((dynamic i) => Event.fromJson(i)).toList();
    return CalendarEvents(events: events);
  }
}

class CalendarCreateEvent {
  CalendarCreateEvent(this.title, this.description, this.startAt, this.endAt);

  final String title;
  final String description;
  final DateTime startAt;
  final DateTime endAt;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'title': title,
      'description': description,
      'start_at': startAt.toUtc().toIso8601String(),
      'end_at': endAt.toUtc().toIso8601String(),
    };
    return data;
  }
}

class CalendarDeleteEvent {
  CalendarDeleteEvent(this.id);

  final String id;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
    };
    return data;
  }
}