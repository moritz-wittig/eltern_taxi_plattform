import 'package:eltern_taxi_plattform/database/todo_db.dart';
import 'package:eltern_taxi_plattform/model/todo.dart';
import 'package:eltern_taxi_plattform/widget/create_todo_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';

class TodosPage extends StatefulWidget {
  const TodosPage({super.key});

  @override
  State<TodosPage> createState() => _TodosPageState();
}

class _TodosPageState extends State<TodosPage>{
  Future<List<Todo>>? futureTodos;
  final todoDB = TodoDB();

  @override
  void initState() {
    super.initState();

    fetchTodos();
  }

  void fetchTodos() {
    setState(() {
      futureTodos = todoDB.fetchAll();
    });
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: const Text("ToDo List"),
    ),
    floatingActionButton: FloatingActionButton(
      child: const Icon(Icons.add),
      onPressed: () {
        showDialog(
          context: context, 
          builder: (_) => CreateTodoWidget(
            onSubmit: (title) async {
              await todoDB.create(title: title);
              if(!mounted) return;
              fetchTodos();
              Navigator.of(context).pop();
            }
          )
        );
      },
    ),
  );
}
