/* 
 * Use TurboC to compile
 * tcc milk.c -o milk.exe
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char * tmpbuf;
static char * strbuf;
static FILE * fp;

typedef struct {
	char * str;
	int num;
}dyn_var_t;
dyn_var_t * stack;
dyn_var_t * var_stack_ptr, * vss_stack_ptr;

int main(void) {
	fp = fopen("milk.txt", "rt");
	tmpbuf = malloc(80);
	strbuf = malloc(80);
	
	stack = malloc(128 * sizeof(dyn_var_t));
	var_stack_ptr = &stack[0];
	vss_stack_ptr = &stack[127];
	
	while(!feof(fp)) {
		const char * buf;
		
		fgets(tmpbuf, 80, fp);
		buf = tmpbuf;
		
		if(!strncasecmp(buf, "pour", 4)) {
			buf += 4 + 1;
			if(!strncasecmp(buf, "yoghurt", 7)) {
				dyn_var_t * var;
				int idx;
				
				buf += 7 + 1;
				
				sscanf(buf, "%d", &idx);
				
				var = &stack[idx];
				
				if(!var->num) {
					printf("(%d)=>%s\n", idx, var->str);
				} else {
					printf("(%d)=>%d\n", idx, var->num);
				}
			}
		} else if(!strncasecmp(buf, "yoghurt", 7)) {
			buf += 7 + 1;
			sscanf(buf, "%[^\n]s", strbuf);
			
			var_stack_ptr->str = malloc(strlen(strbuf));
			strcpy(var_stack_ptr->str, strbuf);
			var_stack_ptr->num = atoi(var_stack_ptr->str);
			
			var_stack_ptr++;
		} else if(!strncasecmp(buf, "cheese", 6)) {
			int idx1, idx2;
			char operator;
			
			buf += 6 + 1;
			sscanf(buf, "%c %d %d", &operator, &idx1, &idx2);
			
			switch(operator) {
			case '+':
				vss_stack_ptr->num = stack[idx1].num + stack[idx2].num;
				break;
			case '-':
				vss_stack_ptr->num = stack[idx1].num - stack[idx2].num;
				break;
			case '/':
				vss_stack_ptr->num = stack[idx1].num / stack[idx2].num;
				break;
			case '*':
				vss_stack_ptr->num = stack[idx1].num * stack[idx2].num;
				break;
			case '^':
				vss_stack_ptr->num = stack[idx1].num ^ stack[idx2].num;
				break;
			case '%':
				vss_stack_ptr->num = stack[idx1].num % stack[idx2].num;
				break;
			case '&':
				vss_stack_ptr->num = stack[idx1].num & stack[idx2].num;
				break;
			case '|':
				vss_stack_ptr->num = stack[idx1].num | stack[idx2].num;
				break;
			default:
				printf("invalid operator %c\n", operator);
				break;
			}
			vss_stack_ptr--;
		} else {
			printf("skipped: %s\n", buf);
		}
	}
	
	free(tmpbuf);
	fclose(fp);
}
