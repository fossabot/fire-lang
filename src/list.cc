#include <vector>
#include <iterator>
#include <algorithm>

template <typename T>
class __fire_list {
public:
	__fire_list operator=(T rhs[]) {
		vect.insert(vect.being(), rhs, sizeof(rhs) / sizeof(*rhs) ); return *this;
	}
	T operator[] (int pos) const {
		return vect[pos];
	}
	T __fire_get(int pos) {
		return vect.at(pos);
	}
	T& operator[](int pos) {
		return vect[pos];
	}
	void __fire_push(T item) {
		vect.push_back(item);
	}
	void __fire_insert(int position, T item) {
		vect.insert(vect.begin() + position, item);
	}
	int __fire_count(T item) {
		int total = 0;
		typename std::vector<T>::const_iterator iter;
		for (iter = vect.begin(); iter != vect.end(); ++iter) {
			if (*iter == item) {
				total++;
			}
		}
		return total;
	}
	void __fire_reverse() {
		std::reverse(vect.begin(), vect.end());
	}
	T __fire_pop(int element=-1) {
		int pos;
		if (element == -1) {
			pos = vect.size() - 1;
		}
		else {
			pos = element;
		}
		T &ret = vect.at(pos);
		vect.erase(vect.begin()+pos);
		return ret;
	}
	int __fire_index(T item) {
		typename std::vector<T>::iterator iter;
		T ret;
		for (iter = vect.begin(); iter != vect.end(); ++iter) {
			if (*iter == item) {
				ret = *iter;
			}
		}
		return ret;
	}
private:
	std::vector<T> vect;
};
