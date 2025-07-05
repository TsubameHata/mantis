export const usePositiveNumber = (v:number = 0)=>{
    const _v1 = ref(v);

    const _v2 = computed({
        get: ()=>_v1.value,
        set: (n:number)=>{
            if(typeof n == "number" && n>=0) _v1.value=n;
        }
    });

    return _v2;
}