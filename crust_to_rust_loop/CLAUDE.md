# Instructions
Use filter_content_by_context to access project files in /workspace in following format:
```bash
filter_content_by_context src/file.rs # use only full relative path to file, the tool works from any arbitrary directory
```
- ```$WORKSPACE_ROOT``` is the root of the project, it is to be used as a mirror of the original project, editing files inside of it, except ```$PROJECT_FILE``` 
wouldn't grant any effect. You may only edit a single file at a time.

- At start-up load y2 plugin skills. 

- ```CLAUDE_PROJECT_ROOT``` is the root of the project, it is to be used as path to set all y2 plugin skills.

- Your task is: remove as many unsafe blocks as possible from $PORTING_FUNCS in $PORTING_FILE

- After finishing the task, you must run test-sqlite to verify your changes.

- If tests fail, you must fix the issues and run tests again.