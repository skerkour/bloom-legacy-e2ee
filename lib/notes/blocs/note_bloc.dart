import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/notes/models/db/note.dart';
import 'package:flutter/material.dart';

class NoteBloc extends BlocBase {
  NoteBloc({@required this.note});

  Note note;

  final StreamController<Note> _noteController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inNote => _noteController.sink;
  Stream<Note> get noteOut => _noteController.stream;

  final StreamController<Note> _noteDeletedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inDeleted => _noteDeletedController.sink;
  Stream<Note> get deleted => _noteDeletedController.stream;

  final StreamController<Note> _noteArchivedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inArchived => _noteArchivedController.sink;
  Stream<Note> get archived => _noteArchivedController.stream;

  final StreamController<Note> _noteUnarchivedController =
      StreamController<Note>.broadcast();
  StreamSink<Note> get _inUnarchived => _noteUnarchivedController.sink;
  Stream<Note> get unarchived => _noteUnarchivedController.stream;

  final StreamController<Color> _colorUpdatedController =
      StreamController<Color>.broadcast();
  StreamSink<Color> get _inColorUpdated => _colorUpdatedController.sink;
  Stream<Color> get color => _colorUpdatedController.stream;

  @override
  void dispose() {
    _noteController.close();
    _noteDeletedController.close();
    _noteArchivedController.close();
    _noteUnarchivedController.close();
    _colorUpdatedController.close();
  }

  Future<void> delete() async {
    if (note.id != null) {
      note = await note.delete();
    }
    _inDeleted.add(note);
  }

  Future<Note> create(String title, String body, Color color) async {
    note = await Note.create(title, body, color);
    _inNote.add(note);
    return note;
  }

  Future<Note> update(Note noteToUpdate) async {
    note = await noteToUpdate.update();
    _inNote.add(note);
    return note;
  }

  Future<void> archive() async {
    note = await note.archive();
    _inNote.add(note);
    _inArchived.add(note);
  }

  Future<void> unarchive() async {
    note = await note.unarchive();
    _inNote.add(note);
    _inUnarchived.add(note);
  }

  Future<void> updateColor(Color color) async {
    note.color = color;
    note = await note.update();
    _inNote.add(note);
    _inColorUpdated.add(color);
  }

  Future<void> save(String title, String body) async {
    note.title = title;
    note.body = body;

    if (note.id == null) {
      if (note.title.isEmpty && note.body.isEmpty) {
        debugPrint('note is empty, aborting');
        return;
      }
      note = await create(note.title, note.body, note.color);
      debugPrint('note created');
    } else {
      note = await update(note);
      debugPrint('note updated');
    }
    _inNote.add(note);
  }
}
